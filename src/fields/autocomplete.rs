use std::rc::Rc;

use clap;
use cursive::views::ViewBox;
use serde_json::value::Value;

use feeders::{DummyFeeder, Feeder};
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
        fields::Field::new(label, AutocompleteManager::with_feeder(feeder), "".to_string())
    }
}

#[derive(Clone)]
pub struct AutocompleteManager {
    feeder: Rc<Feeder>,
    view_factory: Option<Rc<Fn() -> views::Autocomplete>>,
}

impl AutocompleteManager {
    /// Creates an instance with a customized [Feeder].
    ///
    /// If you want to control creation of [views::Autocomplete]
    /// use [with_factory_view].
    ///
    /// [Feeder]: ../../feeders/index.html
    /// [views::Autocomplete]: ../../views/struct.Autocomplete.html
    /// [with_factory_view]: struct.AutocompleteManager.html#method.with_factory_view
    pub fn with_feeder<T: Feeder>(feeder: T) -> Self {
        AutocompleteManager {
            feeder: Rc::new(feeder),
            view_factory: None,
        }
    }
    /// Creates an instance with customized [views::Autocomplete].
    ///
    /// If you want to specify only a [Feeder] (and use a default [views::Autocomplete])
    /// use [with_feeder].
    ///
    /// [Feeder]: ../../feeders/index.html
    /// [views::Autocomplete]: ../../views/struct.Autocomplete.html
    /// [with_feeder]: struct.AutocompleteManager.html#method.with_feeder
    pub fn with_factory_view(factory: Rc<Fn() -> views::Autocomplete>) -> Self {
        AutocompleteManager {
            feeder: Rc::new(DummyFeeder),
            view_factory: Some(factory),
        }
    }

    fn get_view(&self) -> views::Autocomplete {
        let view = if let Some(ref fun) = self.view_factory {
            fun()
        } else {
            views::Autocomplete::new(Rc::clone(&self.feeder))
        };
        view
    }
}

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
        let widget = self.get_view();
        let view = ViewBox::new(Box::new(widget.value(value)));
        view
    }
}

impl fields::FormField for fields::Field<AutocompleteManager, String> {
    fn get_widget_manager(&self) -> &WidgetManager {
        &self.widget_manager
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

    fn get_initial(&self) -> String {
        format!("{}", &self.initial)
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
