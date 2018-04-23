use std::str::FromStr;

use clap;
use cursive::views;
use serde_json::value::Value;

use fields;
use fields::{FieldErrors, WidgetManager};

/// [Field] with [Checkbox view] inside.
///
/// [Field]: struct.Field.html
/// [Checkbox view]: ../../cursive/views/struct.Checkbox.html
pub struct Checkbox;

impl Checkbox {
    /// Creates a new [Field] with [Checkbox view] inside.
    ///
    /// [Field]: struct.Field.html
    /// [Checkbox view]: ../../cursive/views/struct.Checkbox.html
    pub fn new<IS: Into<String>>(label: IS) -> fields::Field {
        let view = views::Checkbox::new();
        fields::Field::new(label, CheckboxManager::new(view))
    }
}

pub struct CheckboxManager {
    view: Option<views::Checkbox>,
}

impl CheckboxManager {
    fn new(view: views::Checkbox) -> CheckboxManager {
        CheckboxManager {
            view: Some(view),
        }
    }
}

impl WidgetManager for CheckboxManager {
    fn take_view(&mut self) -> views::ViewBox {
        views::ViewBox::new(Box::new(self.view.take().unwrap()))
    }

    fn as_string(&self, view_box: &views::ViewBox) -> String {
        let checkbox: &views::Checkbox = (**view_box).as_any().downcast_ref().unwrap();
        let value = checkbox.is_checked();
        format!("{}", value)
    }

    fn set_value(&self, view_box: &mut views::ViewBox, value: &Value) {
        let checkbox: &mut views::Checkbox = (**view_box).as_any_mut().downcast_mut().unwrap();
        let value = value.as_bool().unwrap();
        (*checkbox).set_checked(value);
    }

    fn as_value(&self, view_box: &views::ViewBox) -> Value {
        let checkbox: &views::Checkbox = (**view_box).as_any().downcast_ref().unwrap();
        Value::Bool(checkbox.is_checked())
    }
}
