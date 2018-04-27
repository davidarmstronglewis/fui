use cursive::views::ViewBox;
use serde_json::value::Value;

use feeders::Feeder;
use fields::WidgetManager;
use fields;
use views;

/// [Field] with [Multiselect view] inside.
///
/// [Field]: struct.Field.html
/// [Multiselect view]: ../views/struct.Multiselect.html
pub struct Multiselect;

impl Multiselect {
    /// Creates a new [Field] with [Multiselect view] inside.
    ///
    /// [Field]: struct.Field.html
    /// [Multiselect view]: ../views/struct.Multiselect.html
    pub fn new<IS: Into<String>, F: Feeder>(
        label: IS,
        feeder: F,
    ) -> fields::Field {
        let view = views::Multiselect::new(feeder);
        fields::Field::new(label, MultiselectManager::new(view))
    }
}

pub struct MultiselectManager {
    view: Option<views::Multiselect>,
}

impl MultiselectManager {
    fn new(view: views::Multiselect) -> MultiselectManager {
        MultiselectManager {
            view: Some(view),
        }
    }
}

impl WidgetManager for MultiselectManager {
    fn take_view(&mut self) -> ViewBox {
        ViewBox::new(Box::new(self.view.take().unwrap()))
    }

    fn set_value(&self, view_box: &mut ViewBox, value: &Value) {
        let ms: &mut views::Multiselect = (**view_box).as_any_mut().downcast_mut().unwrap();
        let items: Vec<String> = match *value {
            Value::String(ref v) => vec![v.to_owned()],
            Value::Array(ref v) => {
                v
                    .iter()
                    .map(|i| i.as_str().unwrap().to_owned())
                    .collect()
            }
            _ => vec![],
        };
        ms.select_items(items);
    }

    fn get_value(&self, view_box: &ViewBox) -> Value {
        let ms: &views::Multiselect = (**view_box).as_any().downcast_ref().unwrap();
        let value: Vec<Value> = ms.get_selected_items()
            .iter()
            .map(|x| Value::String((*x).to_owned()))
            .collect();
        Value::Array(value)
    }
}

#[cfg(test)]
mod test_multiselect_validation {
    use super::*;
    use fields::FormField;
    use validators::Required;


    #[test]
    fn required_raised_when_value_missing() {
        let mut field = Multiselect::new("label", vec![""]).validator(Required);

        let result = field.validate();

        assert_eq!(result, Err(vec!["Field is required".to_string()]));
    }
}
