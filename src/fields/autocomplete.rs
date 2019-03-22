use std::rc::Rc;

use clap;
use cursive::views::ViewBox;
use serde_json::value::Value;

use feeders::Feeder;
use fields;
use fields::{FieldErrors, WidgetManager};
use views;

/// Convienient wrapper around `Field<AutocompleteManager, String>`.
pub struct Autocomplete;

impl Autocomplete {
    /// Creates a new `Field<AutocompleteManager, String>`.
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
    fn build_widget(&self, label: &str, help: &str, initial: &str) -> ViewBox {
        let viewbox = self.build_value_view(&initial);
        fields::label_with_help_layout(viewbox, &label, &help)
    }
    fn get_value(&self, view_box: &ViewBox) -> String {
        let view_box = fields::value_view_from_layout(view_box);
        let autocomplete: &views::Autocomplete = (**view_box).as_any().downcast_ref().unwrap();
        let value = (&*(*autocomplete).get_value()).clone();
        value
    }
    fn build_value_view(&self, value: &str) -> ViewBox {
        let view = ViewBox::new(Box::new(
            views::Autocomplete::new(Rc::clone(&self.0)).value(value),
        ));
        view
    }
}

impl fields::FormField for fields::Field<AutocompleteManager, String> {
    fn get_widget_manager(&self) -> &WidgetManager {
        &self.widget_manager
    }
    fn build_widget(&self) -> ViewBox {
        self.widget_manager
            .build_widget(&self.label, &self.help, &self.initial)
    }

    fn validate(&self, data: &str) -> Result<Value, FieldErrors> {
        let mut errors = FieldErrors::new();
        for v in &self.validators {
            if let Some(e) = v.validate(data) {
                errors.push(e);
            }
        }
        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(Value::String(data.to_owned()))
        }
    }

    /// Gets label of the field
    fn get_label(&self) -> &str {
        &self.label
    }

    /// Gets help of the field
    fn get_help(&self) -> &str {
        self.help.as_ref()
    }

    fn clap_arg(&self) -> clap::Arg {
        clap::Arg::with_name(&self.label)
            .help(&self.help)
            .long(&self.label)
            .required(self.is_required())
            .takes_value(true)
    }

    fn clap_args2str(&self, args: &clap::ArgMatches) -> String {
        args.value_of(&self.label).unwrap_or("").to_string()
    }

    fn is_required(&self) -> bool {
        self.is_required()
    }
}
