//! Includes `form's` building blocks, `fields`.
use cursive::view::{View, ViewWrapper};
use cursive::views;
use serde_json::value::Value;
use std::rc::Rc;
use validators::{Required, Validator};

mod autocomplete;
mod checkbox;
mod multiselect;
mod text;

pub use self::autocomplete::Autocomplete;
pub use self::checkbox::Checkbox;
pub use self::multiselect::Multiselect;
pub use self::text::Text;

/// Covers communication from `Field` to `Widget`.
pub trait WidgetManager {
    /// Takes out the inner `View` from this manager.
    ///
    /// # Panic
    /// If called more then once it will panic (bacause `View` is taken).
    fn take_view(&mut self) -> views::ViewBox;
    /// Sets view's value
    fn set_value(&self, view_box: &mut views::ViewBox, value: &Value);
    /// Returns view's value as `Value`
    ///
    /// [serde_json::Value]: ../serde_json/value/enum.Value.html
    fn as_value(&self, view_box: &views::ViewBox) -> Value;
}

/// Container for field's errors
pub type FieldErrors = Vec<String>;

/// Covers communication from `Form` to `Field`.
pub trait FormField: View {
    /// Returns field's labels.
    fn get_label(&self) -> &str;
    /// Gets field's value
    fn get_value(&self) -> Value;
    /// Sets field's value
    fn set_value(&mut self, value: &Value);
    /// Runs field's validators on its data.
    fn validate(&mut self) -> Result<Value, FieldErrors>;
}

/// TODO:: docs
/// Builds container `view` with placeholders for `help`, `value`, `error`.
/// Building block for `Form`s which stores `data` and `Widget`.
/// Widget layout where `label` and `help` are in the same line.
pub struct Field {
    // Label, Help are stored in TextViews
    // if you need &src from TextViews you have to allocate String which will be dropped making
    // your &str invalid, so keep copy of Label, Help in Field to be able to return &src
    label: String,
    help: String,

    validators: Vec<Rc<Validator>>,
    view: views::LinearLayout,
    /// Controlls `View` storing value.
    widget_manager: Box<WidgetManager>,
}
//TODO:: make it macro and use if for CheckboxField, TextField, etc.?
impl Field {
    /// Creates a new `Field` with given `label`, `widget_manager`.
    ///
    /// `label` should match NOTE from [Fui::action]
    ///
    /// [Fui::action]: ../struct.Fui.html#method.action
    pub fn new<VM: WidgetManager + 'static, IS: Into<String>>(label: IS, mut widget_manager: VM) -> Field {
        let label = label.into();
        let label_and_help = views::LinearLayout::horizontal()
            .child(views::TextView::new(label_padding(label.as_ref())))
            .child(views::DummyView)
            .child(views::TextView::new(""));
        let layout = views::LinearLayout::vertical()
                    //TODO:: label can't include separator
                    .child(label_and_help)
                    .child(widget_manager.take_view())
                    .child(views::TextView::new(""))
                    .child(views::DummyView);
        Field {
            label: label,
            help: "".to_string(),
            validators: vec![],
            view: layout,
            widget_manager: Box::new(widget_manager),
        }
    }
    /// Sets initial value of field.
    pub fn initial<IS: Into<Value>>(mut self, initial: IS) -> Self {
        let value = initial.into();
        self.widget_manager.set_value(
            // self.view_value_get_mut() // this makes borrow-checker sad
            self.view.get_child_mut(1).unwrap().as_any_mut().downcast_mut().unwrap(),
            &value);
        self
    }
    /// Sets `help` message for `field`.
    pub fn help(mut self, msg: &str) -> Self {
        self.set_help(msg);
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
    /// Returns view responsible for storing value.
    ///
    /// Returns `ViewBox` since we don't know what `View` is injected.
    fn view_value_get(&self) -> &views::ViewBox {
        self.view.get_child(1).unwrap().as_any().downcast_ref().unwrap()
    }

    /// Returns mutable view responsible for storing value.
    ///
    /// Returns `ViewBox` since we don't know what `View` is injected.
    fn view_value_get_mut(&mut self) -> &mut views::ViewBox {
        self.view.get_child_mut(1).unwrap().as_any_mut().downcast_mut().unwrap()
    }

    /// Returns mutable view responsible for storing label.
    fn view_label_get(&self) -> &views::TextView {
        let label_and_help: &views::LinearLayout = self.view.get_child(0).unwrap().as_any().downcast_ref().unwrap();
        label_and_help.get_child(0).unwrap().as_any().downcast_ref().unwrap()
    }

    /// Returns mutable view responsible for storing help message.
    fn view_help_get_mut(&mut self) -> &mut views::TextView {
        let label_and_help: &mut views::LinearLayout = self.view.get_child_mut(0).unwrap().as_any_mut().downcast_mut().unwrap();
        label_and_help.get_child_mut(2).unwrap().as_any_mut().downcast_mut().unwrap()
    }

    /// Gets help of the field
    pub fn get_help(&self) -> &str {
        &self.help
    }

    /// Sets help message.
    pub fn set_help(&mut self, msg: &str) {
        self.help = msg.to_string();
        let text_view: &mut views::TextView = self.view_help_get_mut();
        text_view.set_content(msg);
    }

    /// Returns mutable view responsible for storing error message.
    fn view_error_get_mut(&mut self) -> &mut views::TextView {
        self.view.get_child_mut(2).unwrap().as_any_mut().downcast_mut().unwrap()
    }

    /// Sets error message.
    pub fn set_error(&mut self, msg: &str) {
        let text_view: &mut views::TextView = self.view_error_get_mut();
        text_view.set_content(msg);
    }

    /// Shows field errors
    fn show_errors(&mut self, errors: &FieldErrors) {
        // TODO: show all errors somehow
        self.set_error(errors.first().map(|e| e.as_ref()).unwrap_or(""));
    }

    fn validate_value(&self, value: &str, errors: &mut FieldErrors) {
        for validator in self.validators.iter() {
            if let Some(e) = validator.validate(&value) {
                errors.push(e);
            }
        }
    }
}

impl FormField for Field {
    /// Gets label of the field.
    fn get_label(&self) -> &str {
        &self.label
    }

    /// Gets value of the field.
    fn get_value(&self) -> Value {
        self.widget_manager.as_value(self.view_value_get())
    }

    /// Sets value of the field.
    fn set_value(&mut self, value: &Value) {
        self.widget_manager.set_value(
            // self.view_value_get_mut(), // this makes borrow-checker sad
            self.view.get_child_mut(1).unwrap().as_any_mut().downcast_mut().unwrap(),
            value,
        );
    }

    /// Validates `Field`.
    fn validate(&mut self) -> Result<Value, FieldErrors> {
        let mut errors: FieldErrors = Vec::new();
        let value = self.widget_manager.as_value(self.view_value_get());
        match value {
            Value::Null => self.validate_value("", &mut errors),
            Value::String(ref value) => {
                self.validate_value(value, &mut errors);
            },
            Value::Array(ref items) => {
                if items.len() == 0 {
                    self.validate_value("", &mut errors);
                } else {
                    for item in items {
                        let text = item.as_str().unwrap();
                        self.validate_value(text, &mut errors);
                    }
                }
            },
            _ => (),

        }
        let result = if errors.len() > 0 {
            self.show_errors(&errors);
            Err(errors)
        } else {
            // clean possibly errors from last validation
            self.show_errors(&vec!["".to_string()]);
            Ok(self.widget_manager.as_value(self.view_value_get()))
        };
        result
    }
}

impl ViewWrapper for Field {
    wrap_impl!(self.view: views::LinearLayout);
}

fn label_padding(label: &str) -> String {
    format!("{:20}", label)
}
