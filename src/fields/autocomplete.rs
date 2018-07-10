use cursive::views::ViewBox;
use serde_json::value::Value;

use feeders::Feeder;
use fields::WidgetManager;
use fields;
use views;

/// [Field] with [Autocomplete view] inside.
///
/// [Field]: struct.Field.html
/// [Autocomplete view]: ../views/struct.Autocomplete.html
pub struct Autocomplete;

impl Autocomplete {
    /// Creates a new [Field] with [Autocomplete view] inside.
    ///
    /// [Field]: struct.Field.html
    /// [Autocomplete view]: ../views/struct.Autocomplete.html
    pub fn new<IS: Into<String>, F: Feeder>(
        label: IS,
        feeder: F,
    ) -> fields::Field {
        let view = views::Autocomplete::new(feeder);
        fields::Field::new(label, AutocompleteManager::new(view), "")
    }
}

pub struct AutocompleteManager {
    view: Option<views::Autocomplete>,
}

impl AutocompleteManager {
    fn new(view: views::Autocomplete) -> AutocompleteManager {
        AutocompleteManager {
            view: Some(view),
        }
    }
}

impl WidgetManager for AutocompleteManager {
    fn take_view(&mut self) -> ViewBox {
        ViewBox::new(Box::new(self.view.take().unwrap()))
    }

    fn set_value(&self, view_box: &mut ViewBox, value: &Value) {
        let ac: &mut views::Autocomplete = (**view_box).as_any_mut().downcast_mut().unwrap();
        let value = match *value {
            Value::Null => "",
            Value::String(ref v) => v,
            Value::Array(ref v) => {
                match v.len() {
                    0 => "",
                    _ => v[0].as_str().unwrap(),
                }
            },
            _ => "",
        };
        (*ac).set_value(value);
    }

    fn get_value(&self, view_box: &ViewBox) -> Value {
        let ac: &views::Autocomplete = (**view_box).as_any().downcast_ref().unwrap();
        let value = (&*(*ac).get_value()).clone();
        Value::String(value.to_owned())
    }
}
