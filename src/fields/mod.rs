//! Includes `form's` building blocks, `fields`.
use clap;
use cursive::view::View;
use cursive::views;
use serde_json::value::Value;
use std::rc::Rc;
use validators::{Required, Validator};

pub(crate) mod autocomplete;
mod checkbox;
pub(crate) mod multiselect;
mod text;

pub use self::autocomplete::Autocomplete;
pub use self::checkbox::Checkbox;
pub use self::multiselect::Multiselect;
pub use self::text::Text;

/// Covers communication from [Field] to [View].
///
/// This [View] is responsible for storing [Field]'s value.
///
/// [Field]: ../fields/index.html
/// [View]: ../views/index.html
pub trait WidgetManager {
    /// Builds a value view.
    fn build_value_view(&self, value: &str) -> views::ViewBox;
    /// Gets view's value.
    fn get_value(&self, view: &views::ViewBox) -> String;
    /// Sets `error` on widget.
    ///
    /// # Note:
    /// WidgetManager doesn't control `error` anymore.
    /// It's been transferred to [FormField]
    ///
    /// [FormField]: ./trait.FormField.html
    #[deprecated(
        since = "1.0.0",
        note = "Errors should be transferred to `Field`. Use `Field.set_error`"
    )]
    // TODO:: rm it
    fn set_error(&self, viewbox: &mut views::ViewBox, error: &str) {
        let layout: &mut views::LinearLayout = (**viewbox).as_any_mut().downcast_mut().unwrap();
        let child: &mut View = (*layout).get_child_mut(2).unwrap();
        let text: &mut views::TextView = (*child).as_any_mut().downcast_mut().unwrap();
        text.set_content(error);
    }
    /// Builds container `view` with placeholders for `help`, `value`, `error`.
    ///
    /// # Note:
    /// WidgetManager doesn't control `help` or `error` anymore.
    /// It's been transferred to [FormField]
    ///
    /// [FormField]: ./trait.FormField.html
    #[deprecated(
        since = "1.0.0",
        note = "Values like `help` or `label` should be stored on `FormField`. Details in documentation of `WidgetManager.build_widget`"
    )]
    // TODO:: rm it
    fn build_widget(&self, label: &str, help: &str, initial: &str) -> views::ViewBox;
}

/// Building block for `Form`s which stores `data` and `Widget`.
#[derive(Clone)]
pub struct Field<W: WidgetManager, T> {
    label: String,
    help: String,
    initial: T,
    validators: Vec<Rc<Validator>>,
    widget_manager: W,
}

impl<W: WidgetManager, T> Field<W, T> {
    /// Creates a new `Field` with given `label`, `widget_manager`, `initial`.
    ///
    /// `label` should match NOTE from [Fui::action]
    ///
    /// [Fui::action]: ../struct.Fui.html#method.action
    pub fn new<IS: Into<String>>(label: IS, widget_manager: W, initial: T) -> Self {
        Field {
            label: label.into(),
            help: "".into(),
            initial: initial,
            validators: vec![],
            widget_manager: widget_manager,
        }
    }
    /// Sets `help` message for `field`.
    pub fn help<IS: Into<String>>(mut self, msg: IS) -> Self {
        self.help = msg.into();
        self
    }
    /// Append `validator`.
    pub fn validator<V: Validator + 'static>(mut self, validator: V) -> Self {
        self.validators.push(Rc::new(validator));
        self
    }
    /// Checks if Field is required
    pub fn is_required(&self) -> bool {
        self.validators
            .iter()
            .any(|&ref x| (**x).as_any().downcast_ref::<Required>().is_some())
    }
}

/// Container for field's errors
pub type FieldErrors = Vec<String>;

/// Covers communication from `Form` to `Field`.
pub trait FormField {
    /// Builds `widget` representing this `field`.
    fn build_widget(&self) -> views::ViewBox {
        let view = self
            .get_widget_manager()
            .build_value_view(&self.get_initial());
        label_with_help_layout(view, self.get_label(), &self.get_help())
    }
    /// Validates `data`.
    fn validate(&self, data: &str) -> Result<Value, FieldErrors>;
    /// Gets `field`'s label.
    fn get_label(&self) -> &str;
    /// Gets `field`'s help
    fn get_help(&self) -> &str;
    /// Gets `initial` value
    fn get_initial(&self) -> String;
    /// Gets manager which controlls `widget`.
    fn get_widget_manager(&self) -> &WidgetManager;
    /// Builds [clap::Arg] needed by automatically generated [clap::App].
    ///
    /// [clap::Arg]: ../../clap/struct.Arg.html
    /// [clap::App]: ../../clap/struct.App.html
    fn clap_arg(&self) -> clap::Arg;
    /// Extracts field's data from [clap::ArgMatches] and converts it to str.
    ///
    /// [clap::App]: ../../clap/struct.ArgMatches.html
    fn clap_args2str(&self, args: &clap::ArgMatches) -> String;
    /// Checks if Field is required
    fn is_required(&self) -> bool;
    /// Sets `error` on widget.
    fn set_error(&self, viewbox: &mut views::ViewBox, error: &str) {
        let layout: &mut views::LinearLayout = (**viewbox).as_any_mut().downcast_mut().unwrap();
        let child: &mut View = (*layout).get_child_mut(2).unwrap();
        let text: &mut views::TextView = (*child).as_any_mut().downcast_mut().unwrap();
        text.set_content(error);
    }
}

fn format_annotation(label: &str, help: &str) -> String {
    if help.len() > 0 {
        format!("{:20}: {}", label, help)
    } else {
        format!("{:20}", label)
    }
}

/// Widget layout where `label` and `help` are in the same line.
pub fn label_with_help_layout(view_box: views::ViewBox, label: &str, help: &str) -> views::ViewBox {
    let text = format_annotation(label, help);
    let widget = views::LinearLayout::vertical()
        .child(views::TextView::new(text))
        .child(view_box)
        .child(views::TextView::new(""))
        .child(views::DummyView);

    views::ViewBox::new(Box::new(widget))
}

/// Finds view storing value in widget layout
pub fn value_view_from_layout(layout: &views::ViewBox) -> &views::ViewBox {
    let layout: &views::LinearLayout = (**layout).as_any().downcast_ref().unwrap();
    let value_view: &View = layout.get_child(1).unwrap();
    (*value_view).as_any().downcast_ref().unwrap()
}
