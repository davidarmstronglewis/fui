use cursive::views::ViewBox;
use cursive::views;
use fields::WidgetManager;
use fields;
use serde_json::value::Value;

/// [Field] with [EditView view] inside.
///
/// [Field]: struct.Field.html
/// [EditView view]: ../../cursive/views/struct.EditView.html
pub struct Text;

impl Text {
    /// Creates a new [Field] with [EditView view] inside.
    ///
    /// [Field]: struct.Field.html
    /// [EditView view]: ../../cursive/views/struct.EditView.html
    pub fn new<IS: Into<String>>(label: IS) -> fields::Field {
        let view = views::EditView::new();
        fields::Field::new(label, EditViewManager::new(view), "")
    }
}

pub struct EditViewManager {
    view: Option<views::EditView>,
}

impl EditViewManager {
    fn new(view: views::EditView) -> EditViewManager {
        EditViewManager {
            view: Some(view),
        }
    }
}

impl WidgetManager for EditViewManager {
    fn take_view(&mut self) -> ViewBox {
        ViewBox::new(Box::new(self.view.take().unwrap()))
    }

    fn set_value(&self, view_box: &mut ViewBox, value: &Value) {
        let edit_view: &mut views::EditView = (**view_box).as_any_mut().downcast_mut().unwrap();
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
        (*edit_view).set_content(value);
    }

    fn get_value(&self, view_box: &ViewBox) -> Value {
        let edit_view: &views::EditView = (**view_box).as_any().downcast_ref().unwrap();
        let value = (&*(*edit_view).get_content()).clone();
        Value::String(value.to_owned())
    }
}
