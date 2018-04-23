use std::rc::Rc;

use clap;
use cursive::view::View;
use cursive::views::ViewBox;
use serde_json::value::Value;

use feeders::Feeder;
use fields::{FieldErrors, WidgetManager};
use fields;
use views;

const VALUE_SEP: &'static str = ",";

/// Convienient wrapper around `Field<AutocompleteManager, String>`.
pub struct Autocomplete;

impl Autocomplete {
    /// Creates a new `Field<AutocompleteManager, String>`.
    pub fn new<IS: Into<String>, F: Feeder>(
        label: IS,
        feeder: F,
    ) -> Field2 {
        let view = views::Autocomplete::new(feeder);
        Field2::new(label, AutocompleteManager::new(view))
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



//TODO::: cleanups
use cursive::view::ViewWrapper;
use cursive::views::{LinearLayout, TextView, DummyView};
use validators::{Required, Validator};
//TODO::: rename to Field/Autocomplete/or whatever
//TODO::: mv Field to fields/mod.rs
/// TODO::: docs
/// Builds container `view` with placeholders for `help`, `value`, `error`.
pub struct Field2 {
    // TODO::: explain why these fields
    label: String,
    help: String,

    validators: Vec<Rc<Validator>>,
    view: LinearLayout,
    /// Controlls `View` storing value.
    widget_manager: AutocompleteManager,
}
//TODO::: make it macro and use if for CheckboxField, TextField, etc.?
impl Field2 {
    fn new<IS: Into<String>>(label: IS, mut widget_manager: AutocompleteManager) -> Field2 {
        let label = label.into();
        let label_and_help = LinearLayout::horizontal()
            .child(TextView::new(label_padding(label.as_ref())))
            .child(DummyView)
            .child(TextView::new(""));
        let layout = LinearLayout::vertical()
                    //TODO:: label can't include separator
                    .child(label_and_help)
                    .child(widget_manager.take_view())
                    .child(TextView::new(""))
                    .child(DummyView);
        Field2 {
            label: label,
            help: "".to_string(),
            validators: vec![],
            view: layout,
            widget_manager: widget_manager,
        }
    }
    /// Sets initial value of field.
    pub fn initial<IS: Into<String>>(mut self, initial: IS) -> Self {
        let value = initial.into();
        self.widget_manager.set_value(
            // self.view_value_get_mut() // this makes borrow-checker sad
            self.view.get_child_mut(1).unwrap().as_any_mut().downcast_mut().unwrap(),
            value.as_ref());
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
        //TODO:::
        //self.validators
        //    .iter()
        //    .any(|&ref x| (**x).as_any().downcast_ref::<Required>().is_some())
        true
    }
    /// Returns view responsible for storing value.
    ///
    /// Returns `ViewBox` since we don't know what `View` is injected.
    fn view_value_get(&self) -> &ViewBox {
        self.view.get_child(1).unwrap().as_any().downcast_ref().unwrap()
    }

    /// Returns mutable view responsible for storing value.
    ///
    /// Returns `ViewBox` since we don't know what `View` is injected.
    fn view_value_get_mut(&mut self) -> &mut ViewBox {
        self.view.get_child_mut(1).unwrap().as_any_mut().downcast_mut().unwrap()
    }

    /// Returns mutable view responsible for storing label.
    fn view_label_get(&self) -> &TextView {
        let label_and_help: &LinearLayout = self.view.get_child(0).unwrap().as_any().downcast_ref().unwrap();
        label_and_help.get_child(0).unwrap().as_any().downcast_ref().unwrap()
    }

    /// Returns mutable view responsible for storing help message.
    fn view_help_get_mut(&mut self) -> &mut TextView {
        let label_and_help: &mut LinearLayout = self.view.get_child_mut(0).unwrap().as_any_mut().downcast_mut().unwrap();
        label_and_help.get_child_mut(2).unwrap().as_any_mut().downcast_mut().unwrap()
    }

    /// Gets help of the field
    fn get_help(&self) -> &str {
        &self.help
    }

    /// Sets help message.
    pub fn set_help(&mut self, msg: &str) {
        self.help = msg.to_string();
        let text_view: &mut TextView = self.view_help_get_mut();
        text_view.set_content(msg);
    }

    /// Returns mutable view responsible for storing error message.
    fn view_error_get_mut(&mut self) -> &mut TextView {
        self.view.get_child_mut(2).unwrap().as_any_mut().downcast_mut().unwrap()
    }

    /// Sets error message.
    pub fn set_error(&mut self, msg: &str) {
        let text_view: &mut TextView = self.view_error_get_mut();
        text_view.set_content(msg);
    }

    /// Shows field errors
    fn show_errors(&mut self, errors: &FieldErrors) {
        // TODO: show all errors somehow
        self.set_error(errors.first().map(|e| e.as_ref()).unwrap_or(""));
    }

}
//TODO::: redefine FormField trait after cleanups
impl fields::FormField for Field2 {
    /// Validates `Field`.
    fn validate(&mut self) -> Result<Value, FieldErrors> {
        let mut errors: FieldErrors = Vec::new();
        let value = self.widget_manager.as_string(self.view_value_get());
        for v in self.validators.iter() {
            if let Some(e) = v.validate(&value) {
                errors.push(e);
            }
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
    /// Gets label of the field
    fn get_label(&self) -> &str {
        &self.label
    }

    /// Builds [clap::Arg] needed by automatically generated [clap::App].
    ///
    /// [clap::Arg]: ../../clap/struct.Arg.html
    /// [clap::App]: ../../clap/struct.App.html
    //TODO::: make it trait?
    // TODO::: rename it: fn as_clap_arg(&self) -> clap::Arg {
    fn clap_arg(&self) -> clap::Arg {
        let (multiple, takes_value) = match self.widget_manager.as_value(self.view_value_get()) {
            Value::Number(_) => (false, true),
            Value::String(_) => (false, true),
            Value::Array(_) => (true, true),
            _ => (false, false),
        };
        //TODO::: &self.label is enough
        clap::Arg::with_name(self.get_label())
            .help(self.get_help())
            //TODO::: &self.label is enough
            .long(self.get_label())
            .required(self.is_required())
            .multiple(multiple)
            .takes_value(takes_value)
    }

    fn clap_args2str(&self, args: &clap::ArgMatches) -> String {
        match self.widget_manager.as_value(self.view_value_get()) {
            Value::Bool(_) => {
                let v = if args.is_present(&self.label) {
                    "true"
                } else {
                    "false"
                };
                v.to_string()
            },
            Value::Number(_) | Value::String(_) => {
                args.value_of(&self.label).unwrap_or("").to_string()
            },
            Value::Array(_) => {
                let values = args.values_of(&self.label)
                    .unwrap_or(clap::Values::default());
                values.collect::<Vec<&str>>().join(VALUE_SEP)
            },
            _ => "".to_string(),
        }
    }

    fn set_value(&mut self, value: &str) {
        self.widget_manager.set_value(
            // self.view_value_get_mut(), // this makes borrow-checker sad
            self.view.get_child_mut(1).unwrap().as_any_mut().downcast_mut().unwrap(),
            value,
        );
    }

}
impl ViewWrapper for Field2 {
    wrap_impl!(self.view: LinearLayout);
}
fn label_padding(label: &str) -> String {
    format!("{:20}", label)
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
//        let field = Field2::new("label", AutocompleteManager::new(views::Autocomplete::new(vec![""])));
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
//        let field = Field2::new("label", AutocompleteManager::new(views::Autocomplete::new(vec!["value"])));
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
//        let field = Field2::new("label", Multivalue::new(views::Multivalue::new(vec!["value"])));
//
//        let cmd_with_true = app.get_matches_from(vec!["myprog", "--multivalue-arg", "value", "value2"]);
//
//        assert_eq!(field.clap_args2value(cmd_with_true), Value::Array(["value", "value2"]));
//    }
//}
