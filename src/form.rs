//! Contains `form` related concetps like `FormView`.
use std::rc::Rc;
use std::collections::HashMap;

use clap;
use cursive::Cursive;
use cursive::event::{Callback, Event, EventResult, Key, MouseButton, MouseEvent};
use cursive::view::{View, ViewWrapper};
use cursive::views::{Dialog, DialogFocus, LinearLayout, ViewBox};
use serde_json::map::Map;
use serde_json::value::Value;

use fields::{Field, FieldErrors, FormField};


/// Container for form's errors
pub type FormErrors = HashMap<String, FieldErrors>;


type OnSubmit = Option<Rc<Fn(&mut Cursive, Value)>>;
type OnCancel = Option<Rc<Fn(&mut Cursive)>>;

/// Aggregates `fields` and handles process of `submitting` (or `canceling`).
pub struct FormView {
    view: Dialog,
    field_count: u8,

    on_submit: OnSubmit,
    on_cancel: OnCancel,
}
impl FormView {
    /// Creates a new `FormView` with two buttons `submit` and `cancel`.
    //TODO: take name & desc + name exposed as title
    pub fn new() -> Self {
        let layout = Dialog::new()
            .content(LinearLayout::vertical())
            .button("Cancel", |_| {})
            .button("Submit (Ctrl+f)", |_| {});
        FormView {
            view: layout,
            field_count: 0,

            on_submit: None,
            on_cancel: None,
        }
    }

    /// Appends `field` to field list.
    pub fn field<V: FormField + 'static>(mut self, field: V) -> Self {
        self.field_count += 1;
        self.view
            .get_content_mut()
            .as_any_mut()
            .downcast_mut::<LinearLayout>()
            .unwrap()
            .add_child(field);
        self
    }

    /// Sets the function to be called when submit is triggered.
    pub fn set_on_submit<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cursive, Value) + 'static,
    {
        self.on_submit = Some(Rc::new(callback));
    }

    /// Sets the function to be called when submit is triggered.
    ///
    /// Chainable variant.
    pub fn on_submit<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut Cursive, Value) + 'static,
    {
        self.set_on_submit(callback);
        self
    }

    /// Sets the function to be called when cancel is triggered.
    pub fn set_on_cancel<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cursive) + 'static,
    {
        self.on_cancel = Some(Rc::new(callback));
    }

    /// Sets the function to be called when cancel is triggered.
    ///
    /// Chainable variant.
    pub fn on_cancel<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut Cursive) + 'static,
    {
        self.set_on_cancel(callback);
        self
    }

    /// Translates form's fields to [clap::Arg].
    ///
    /// [clap::Arg]: ../../clap/struct.Arg.html
    pub fn as_clap_args(&self) -> Vec<clap::Arg> {
        let mut args = Vec::with_capacity(self.field_count as usize);
        // TODO::: this needs proper iteration or iterator
        for idx in 0..self.field_count {
            let view: &View = self.view
                .get_content()
                .as_any()
                .downcast_ref::<LinearLayout>()
                .unwrap()
                .get_child(idx as usize).unwrap();
            let field: &Field = view.as_any().downcast_ref().unwrap();
            let arg = field.as_clap_arg();
            args.push(arg);
        }
        return args;
    }

    /// Translates [clap::ArgMatches] to [serde_json::Value] based on fields.
    ///
    /// [clap::ArgMatches]: ../../clap/struct.ArgMatches.html
    /// [serde_json::Value]: ../../serde_json/enum.Value.html
    //TODO::: rename it to clap_args_deser?
    pub fn clap_arg_matches2value(&self, arg_matches: &clap::ArgMatches) -> HashMap<String, String> {
        let mut form_data = HashMap::with_capacity(self.field_count as usize);
        // TODO::: this needs proper iteration or iterator
        for idx in 0..self.field_count {
            let view: &View = self.view
                .get_content()
                .as_any()
                .downcast_ref::<LinearLayout>()
                .unwrap()
                .get_child(idx as usize).unwrap();
            let field: &Field = view.as_any().downcast_ref().unwrap();
            let data = field.clap_args2str(&arg_matches);
            form_data.insert(field.get_label().to_owned(), data);
        }
        form_data
    }

    /// Validates form.
    pub fn validate(&mut self) -> Result<Value, FormErrors> {
        let mut data = Map::with_capacity(self.field_count as usize);
        let mut errors: FormErrors = HashMap::with_capacity(self.field_count as usize);
        // TODO::: this needs proper iteration or iterator
        for idx in 0..self.field_count {
            let view: &mut View = self.view
                .get_content_mut()
                .as_any_mut()
                .downcast_mut::<LinearLayout>()
                .unwrap()
                .get_child_mut(idx as usize).unwrap();
            let field: &mut Field = view.as_any_mut().downcast_mut().unwrap();
            let label = field.get_label().to_string();
            match field.validate() {
                Ok(v) => {
                    data.insert(label, v);
                }
                Err(e) => {
                    errors.insert(label, e);
                }
            }
        }
        match errors.is_empty() {
            true => Ok(Value::Object(data)),
            false => Err(errors),
        }
    }

    ///TODO::: docs
    //TODO::: return errors?
    //TODO::: make form_data own type + use it in clap_arg_matches2value?
    pub fn set_data(&mut self, form_data: HashMap<String, String>) {
        eprintln!("FORM_DATA {:?}", form_data);
        use fields::WidgetManager;
        // TODO::: this needs proper iteration or iterator
        for idx in 0..self.field_count {
            let view: &mut View = self.view
                .get_content_mut()
                .as_any_mut()
                .downcast_mut::<LinearLayout>()
                .unwrap()
                .get_child_mut(idx as usize).unwrap();
            let field: &mut Field = view.as_any_mut().downcast_mut().unwrap();
            let label = field.get_label().to_string();
            field.set_value(form_data.get(&label).unwrap());
        }
    }

    fn event_submit(&mut self) -> EventResult {
        match self.validate() {
            Ok(data_map) => {
                let opt_cb = self.on_submit
                    .clone()
                    .map(|cb| Callback::from_fn(move |c| cb(c, data_map.clone())));
                EventResult::Consumed(opt_cb)
            }
            Err(_) => {
                // TODO: the event focus next required/invalid field?
                EventResult::Consumed(None)
            }
        }
    }

    fn event_cancel(&mut self) -> EventResult {
        let cb = self.on_cancel
            .clone()
            .map(|cb| Callback::from_fn(move |c| cb(c)));
        EventResult::Consumed(cb)
    }

    /// Sets `title` of the form on the top of it.
    pub fn title(mut self, title: &str) -> Self {
        self.view.set_title(title);
        self
    }
}

impl ViewWrapper for FormView {
    wrap_impl!(self.view: Dialog);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Mouse {
                offset: _,
                position: _,
                event: MouseEvent::Press(btn),
            } => {
                if btn == MouseButton::Left {
                    self.with_view_mut(|v| v.on_event(event))
                        .unwrap_or(EventResult::Ignored);
                    match self.view.focus() {
                        DialogFocus::Button(0) => self.event_cancel(),
                        DialogFocus::Button(1) => self.event_submit(),
                        _ => EventResult::Ignored,
                    }
                } else {
                    EventResult::Ignored
                }
            }
            Event::Key(Key::Enter) => match self.view.focus() {
                DialogFocus::Button(0) => self.event_cancel(),
                DialogFocus::Button(1) => self.event_submit(),
                _ => self.with_view_mut(|v| v.on_event(event))
                    .unwrap_or(EventResult::Ignored),
            },
            // TODO: ctlr+enter binding?
            Event::CtrlChar('f') => self.event_submit(),
            _ => {
                // default behaviour from ViewWrapper
                self.with_view_mut(|v| v.on_event(event))
                    .unwrap_or(EventResult::Ignored)
            }
        }
    }
}
