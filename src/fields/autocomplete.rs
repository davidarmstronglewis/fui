use std::rc::Rc;

use cursive::view::AnyView;
use cursive::views::{LinearLayout, TextView};
use serde_json::value::Value;

use feeders::Feeder;
use fields::WidgetManager;
use fields;
use views;

pub struct Autocomplete;

impl Autocomplete {
    pub fn new<IS: Into<String>, F: Feeder>(
        label: IS,
        feeder: F,
    ) -> fields::Field<AutocompleteManager, String> {
        fields::Field::new(label, AutocompleteManager(Rc::new(feeder)), "".to_string())
    }
}

#[derive(Clone)]
pub struct AutocompleteManager(Rc<Feeder>);

impl WidgetManager for AutocompleteManager {
    fn full_widget(&self, label: &str, help: &str, initial: &str) -> Box<AnyView> {
        let view = self.widget_factory(&initial);
        fields::label_with_help_layout(view, &label, &help)
    }
    fn get_value(&self, view: &AnyView) -> String {
        let boxed_widget = (*view).as_any().downcast_ref::<Box<AnyView>>().unwrap();
        let widget = (**boxed_widget)
            .as_any()
            .downcast_ref::<LinearLayout>()
            .unwrap();
        let boxed_field = (*widget)
            .get_child(1)
            .unwrap()
            .as_any()
            .downcast_ref::<Box<AnyView>>()
            .unwrap();
        let ac = (**boxed_field)
            .as_any()
            .downcast_ref::<views::Autocomplete>()
            .unwrap();
        let value = (*ac).get_value();

        (&*value).clone()
    }
    fn set_error(&self, view: &mut AnyView, error: &str) {
        let boxed_widget = (*view).as_any_mut().downcast_mut::<Box<AnyView>>().unwrap();
        let widget = (**boxed_widget)
            .as_any_mut()
            .downcast_mut::<LinearLayout>()
            .unwrap();
        let error_field = (*widget)
            .get_child_mut(2)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<TextView>()
            .unwrap();
        error_field.set_content(error);
    }
    fn widget_factory(&self, value: &str) -> Box<AnyView> {
        Box::new(views::Autocomplete::new(Rc::clone(&self.0)).value(value))
    }
}

impl fields::FormField for fields::Field<AutocompleteManager, String> {
    fn get_widget(&self) -> Box<AnyView> {
        self.widget_manager
            .full_widget(&self.label, &self.help, &self.initial)
    }

    fn get_widget_value(&self, view: &AnyView) -> String {
        self.widget_manager.get_value(view)
    }

    fn validate(&self, data: &str) -> Result<Value, String> {
        for v in &self.validators {
            if let Some(e) = v.validate(data) {
                return Err(e);
            }
        }
        Ok(Value::String(data.to_owned()))
    }

    /// Gets label of the field
    fn get_label(&self) -> &str {
        &self.label
    }

    /// Sets field's error
    fn set_widget_error(&self, view: &mut AnyView, error: &str) {
        self.widget_manager.set_error(view, error)
    }
}
