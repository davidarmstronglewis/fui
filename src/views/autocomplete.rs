use std::rc::Rc;

use cursive::Cursive;
use cursive::With;
use cursive::event::{Callback, Event, EventResult, Key};
use cursive::traits::View;
use cursive::view::ViewWrapper;
use cursive::views::{EditView, LinearLayout, SelectView};
use serde_json::value::Value;

use feeders::Feeder;
use super::is_value_from_select;

// TODO: better performance while typing

pub type OnSubmit = Option<Rc<Fn(&mut Cursive, Rc<String>)>>;

/// Single selection view with suggestions
pub struct Autocomplete {
    view: LinearLayout,

    feeder: Rc<Feeder>,
    shown_count: u8,
    submit_anything: bool,
    suggestion_offset: usize,
    // User typed text handled manually (EditView content is changing by selection)
    typed_value: Rc<String>,

    on_submit: OnSubmit,
}

impl Autocomplete {
    /// Creates a new `Autocomplete` with passed `feeder`
    pub fn new<T: Feeder>(feeder: T) -> Autocomplete {
        let shown_count = 5;

        let select = SelectView::<String>::new()
                .with_all_str(feeder.query("", 0, shown_count).into_iter())
                //TODO: make fixed height for select equal to shown_count
                // use cursive::traits::Boxable;
                //.fixed_height(shown_count)
                // * using fixed_height converts SelectView to BoxView
                // * each shown_count update should update size of select (which'd be BoxView)
                ;

        let layout = LinearLayout::vertical()
            .child(EditView::new())
            .child(select);

        let ac = Autocomplete {
            view: layout,

            feeder: Rc::new(feeder),
            shown_count: shown_count as u8,
            submit_anything: false,
            suggestion_offset: 0usize,
            typed_value: Rc::new("".to_string()),

            on_submit: None,
        };

        ac
    }

    /// Gets typed in value.
    pub fn get_value(&self) -> Rc<String> {
        self.get_edit_view().get_content()
    }

    /// Sets typed in value.
    pub fn set_value(&mut self, value: &str) {
        self.get_edit_view_mut().set_content(value);
    }

    /// Allows to submit any text.
    pub fn submit_anything(mut self) -> Self {
        self.submit_anything = true;
        self
    }

    /// Sets text value.
    pub fn value(mut self, initial: &str) -> Self {
        self.get_edit_view_mut().set_content(initial);
        self.refresh_listing();
        self
    }

    /// Refresh suggestions.
    fn refresh_listing(&mut self) {
        let feeder = Rc::clone(&self.feeder);
        let text = self.get_edit_view().get_content();
        let shown_count = self.shown_count as usize;
        let select = self.get_select_view_mut();
        select.clear();
        select.add_all_str((*feeder).query(text.as_ref(), 0, shown_count).into_iter());
    }

    /// Copy selected text to edit view.
    fn selection_to_edit(&mut self) {
        if !self.get_select_view().is_empty() {
            let selection = self.get_select_view_mut().selection();
            self.get_edit_view_mut().set_content((&*selection).clone());
        }
    }

    /// Checks if value comes from suggestions.
    pub fn is_value_from_select(&self, to_check: &str) -> bool {
        let select = self.get_select_view();
        is_value_from_select(select, to_check)
    }

    fn get_edit_view(&self) -> &EditView {
        self.view
            .get_child(0)
            .unwrap()
            .as_any()
            .downcast_ref::<EditView>()
            .unwrap()
    }

    fn get_edit_view_mut(&mut self) -> &mut EditView {
        self.view
            .get_child_mut(0)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<EditView>()
            .unwrap()
    }

    fn get_select_view(&self) -> &SelectView {
        self.view
            .get_child(1)
            .unwrap()
            .as_any()
            .downcast_ref::<SelectView<String>>()
            .unwrap()
    }

    fn get_select_view_mut(&mut self) -> &mut SelectView {
        self.view
            .get_child_mut(1)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<SelectView<String>>()
            .unwrap()
    }

    fn load_data(&mut self) -> bool {
        let shown_count = self.shown_count as usize;
        let feeder = Rc::clone(&self.feeder);
        let typed_value = &*self.typed_value.clone();
        let data = (*feeder).query(typed_value, self.suggestion_offset, shown_count);
        if data.len() == shown_count {
            let select = self.get_select_view_mut();
            select.clear();
            select.add_all_str(data.into_iter());
            true
        } else {
            false
        }
    }

    fn is_nth(&mut self, nth: usize) -> bool {
        match self.get_select_view().selected_id() {
            Some(v) => v == nth,
            None => false,
        }
    }

    fn is_top(&mut self) -> bool {
        self.is_nth(0)
    }

    fn is_bottom(&mut self) -> bool {
        let last_idx = self.shown_count as usize - 1;
        self.is_nth(last_idx)
    }

    fn select_up_was_top(&mut self) -> bool {
        let is_top_before_select = self.is_top();
        self.get_select_view_mut().select_up(1);
        return is_top_before_select;
    }

    fn select_down_was_bottom(&mut self) -> bool {
        let is_bottom_before_selected = self.is_bottom();
        self.get_select_view_mut().select_down(1);
        is_bottom_before_selected
    }

    fn scroll_up(&mut self) {
        if self.get_select_view().is_empty() {
            return;
        }
        let was_top = self.select_up_was_top();
        if was_top {
            self.suggestion_offset = self.suggestion_offset.saturating_sub(1);
            self.load_data();
        }
        self.selection_to_edit();
    }

    fn scroll_down(&mut self) {
        if self.get_select_view().is_empty() {
            return;
        }
        let last_idx = self.shown_count as usize - 1;
        let was_bottom = self.select_down_was_bottom();
        if was_bottom {
            self.suggestion_offset += 1;
            if self.load_data() {
                self.get_select_view_mut().set_selection(last_idx);
            } else {
                self.suggestion_offset -= 1;
            }
        }
        self.selection_to_edit();
    }

    /// Sets the function to be called when submit is triggered.
    pub fn set_on_submit<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cursive, Rc<String>) + 'static,
    {
        self.on_submit = Some(Rc::new(callback));
    }

    /// Sets the function to be called when submit is triggered.
    ///
    /// Chainable variant.
    pub fn on_submit<F>(self, callback: F) -> Self
    where
        F: Fn(&mut Cursive, Rc<String>) + 'static,
    {
        self.with(|v| v.set_on_submit(callback))
    }

    fn is_edit_focused(&self) -> bool {
        self.view.get_focus_index() == 0
    }

    fn is_select_focused(&self) -> bool {
        self.view.get_focus_index() == 1
    }

    fn focus_edit(&mut self) {
        if !self.is_edit_focused() {
            let tab_event = Event::Shift(Key::Tab);
            self.with_view_mut(|v| v.on_event(tab_event));
        }
    }

    fn focus_select(&mut self) {
        if !self.is_select_focused() {
            let tab_event = Event::Key(Key::Tab);
            self.with_view_mut(|v| v.on_event(tab_event));
        }
    }
}

impl ViewWrapper for Autocomplete {
    wrap_impl!(self.view: LinearLayout);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char(_) | Event::Key(Key::Backspace) | Event::Key(Key::Del) => {
                // typing
                self.focus_edit();
                self.with_view_mut(|v| v.on_event(event))
                    .unwrap_or(EventResult::Ignored);
                self.typed_value = self.get_edit_view().get_content();
                self.suggestion_offset = 0;
                self.refresh_listing();
                EventResult::Consumed(None)
            }
            Event::CtrlChar('u') => {
                self.focus_edit();
                self.get_edit_view_mut().set_content("");
                self.typed_value = Rc::new("".to_string());
                self.suggestion_offset = 0;
                self.refresh_listing();
                EventResult::Consumed(None)
            }
            Event::Key(Key::Down) | Event::CtrlChar('n') => {
                if self.is_edit_focused() && self.is_top() {
                    // allow to select first element
                    self.selection_to_edit();
                } else {
                    self.scroll_down();
                }
                self.focus_select();
                EventResult::Consumed(None)
            }
            Event::Key(Key::Up) | Event::CtrlChar('p') => {
                // move selection up
                self.scroll_up();
                self.focus_select();
                EventResult::Consumed(None)
            }
            Event::Key(Key::Enter) => {
                // submitting
                self.with_view_mut(|v| v.on_event(event))
                    .unwrap_or(EventResult::Ignored);

                let to_submit = self.get_edit_view().get_content();

                if !self.submit_anything {
                    let from_select = self.is_value_from_select(&*to_submit);
                    if !from_select {
                        return EventResult::Ignored;
                    }
                }

                let cb = self.on_submit
                    .clone()
                    .map(|on_submit| Callback::from_fn(move |c| on_submit(c, to_submit.clone())));
                EventResult::Consumed(cb)
            }
            _ => {
                // default behaviour from ViewWrapper
                self.with_view_mut(|v| v.on_event(event))
                    .unwrap_or(EventResult::Ignored)
            }
        }
    }
}
