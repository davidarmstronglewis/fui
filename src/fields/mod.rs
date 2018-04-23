//! Includes `form's` building blocks, `fields`.
use clap;
use cursive::view::View;
use cursive::views;
use serde_json::value::Value;
use std::rc::Rc;
use validators::{Required, Validator};

mod autocomplete;
////TODO::: uncomment
//mod checkbox;
//mod multiselect;
//mod text;

pub use self::autocomplete::Autocomplete;
pub use self::autocomplete::Field2;
////TODO::: uncomment
//pub use self::checkbox::Checkbox;
//pub use self::multiselect::Multiselect;
//pub use self::text::Text;

/// Covers communication from `Field` to `Widget`.
pub trait WidgetManager {
    /// Takes out the inner `View` from this manager.
    ///
    /// # Panic
    /// If called more then once it will panic (bacause `View` is taken).
    fn take_view(&mut self) -> views::ViewBox;
    /// Sets view's value
    fn set_value(&self, view_box: &mut views::ViewBox, value: &str);
    /// Returns view's value as `String`
    fn as_string(&self, view_box: &views::ViewBox) -> String;
    /// Returns view's value as `Value`
    ///
    /// [serde_json::Value]: ../serde_json/value/enum.Value.html
    fn as_value(&self, view_box: &views::ViewBox) -> Value;
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
pub trait FormField: View {
    /// Gets `field`'s label.
    fn get_label(&self) -> &str;
    /// Builds [clap::Arg] needed by automatically generated [clap::App].
    ///
    /// [clap::Arg]: ../../clap/struct.Arg.html
    /// [clap::App]: ../../clap/struct.App.html
    //TODO::: rename it as_clap_arg
    fn clap_arg(&self) -> clap::Arg;
    /// Extracts field's data from [clap::ArgMatches] and converts it to str.
    ///
    /// [clap::App]: ../../clap/struct.ArgMatches.html
    fn clap_args2str(&self, args: &clap::ArgMatches) -> String;
    /// Runs validators on field data
    fn validate(&mut self) -> Result<Value, FieldErrors>;
    /// Sets view's value
    fn set_value(&mut self, value: &str);
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
