use std::rc::Rc;

use clap;
use cursive::view::View;
use cursive::views::ViewBox;
use serde_json::value::Value;

use feeders::Feeder;
use fields::{FieldErrors, WidgetManager};
use fields;
use views;

/// Convienient wrapper around `Field<AutocompleteManager, String>`.
pub struct Autocomplete;

impl Autocomplete {
    /// Creates a new `Field<AutocompleteManager, String>`.
    pub fn new<IS: Into<String>, F: Feeder>(
        label: IS,
        feeder: F,
    ) -> fields::Field {
        let view = views::Autocomplete::new(feeder);
        fields::Field::new(label, AutocompleteManager::new(view))
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
    fn as_string(&self, view_box: &ViewBox) -> String {
        let ac: &views::Autocomplete = (**view_box).as_any().downcast_ref().unwrap();
        let value = (&*(*ac).get_value()).clone();
        value
    }
    fn set_value(&self, view_box: &mut ViewBox, value: &str) {
        let ac: &mut views::Autocomplete = (**view_box).as_any_mut().downcast_mut().unwrap();
        (*ac).set_value(value);
    }

    fn as_value(&self, view_box: &ViewBox) -> Value {
        let ac: &views::Autocomplete = (**view_box).as_any().downcast_ref().unwrap();
        let value = (&*(*ac).get_value()).clone();
        Value::String(value.to_owned())
    }
}




// TODO::: use it or remove it
//#[cfg(test)]
//mod clap_args_conversion {
//    use super::*;
//    use clap::ArgMatches;
//
//    #[test]
//    fn value_is_bool_when_arg_is_switch() {
//        let app = clap::App::new("myprog")
//            .arg(clap::Arg::with_name("checkbox"));
//        let field = Field::new("label", AutocompleteManager::new(views::Autocomplete::new(vec![""])));
//
//        let cmd_with_true = app.get_matches_from(vec!["myprog", "--checkbox"]);
//
//        assert_eq!(field.clap_args2value(cmd_with_true), Value::Bool(true)
//        );
//        assert_eq!(field.clap_args2value(cmd_with_true), Value::Bool(false));
//    }
//
//    #[test]
//    fn value_is_string_when_arg_is_single_value() {
//        let app = clap::App::new("myprog")
//            .arg(clap::Arg::with_name("autocomplete-arg"));
//        let field = Field::new("label", AutocompleteManager::new(views::Autocomplete::new(vec!["value"])));
//
//        let cmd_with_true = app.get_matches_from(vec!["myprog", "--autocomplete-arg", "value"]);
//
//        assert_eq!(field.clap_args2value(cmd_with_true), Value::String("value"));
//    }
//
//    #[test]
//    fn value_is_array_when_arg_is_multi_value() {
//        let app = clap::App::new("myprog")
//            .arg(clap::Arg::with_name("multivalue-arg"));
//        let field = Field::new("label", Multivalue::new(views::Multivalue::new(vec!["value"])));
//
//        let cmd_with_true = app.get_matches_from(vec!["myprog", "--multivalue-arg", "value", "value2"]);
//
//        assert_eq!(field.clap_args2value(cmd_with_true), Value::Array(["value", "value2"]));
//    }
//}
