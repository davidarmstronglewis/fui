use std::ops::Deref;
use std::rc::Rc;

use clap;
use cursive::views::ViewBox;
use serde_json::value::Value;

use feeders::{DummyFeeder, Feeder};
use fields;
use fields::{label_with_help_layout, Field, FieldErrors, FormField, WidgetManager};
use views;

const VALUE_SEP: &'static str = ",";

/// Convienient wrapper around `Field<MultiselectManager, Vec<String>>`.
pub struct Multiselect;

impl Multiselect {
    /// Creates a new `Field<MultiselectManager, Vec<String>>`.
    pub fn new<IS: Into<String>, F: Feeder>(
        label: IS,
        feeder: F,
    ) -> Field<MultiselectManager, Vec<String>> {
        let mngr = MultiselectManager::with_feeder(feeder);
        Field::new(label, mngr, Vec::new())
    }
}

#[derive(Clone)]
pub struct MultiselectManager {
    feeder: Rc<Feeder>,
    view_factory: Option<Rc<Fn() -> views::Multiselect>>,
}

impl MultiselectManager {
    /// Creates an instance with a customized [Feeder].
    ///
    /// If you want to control creation of [views::Multiselect]
    /// use [with_factory_view]
    ///
    /// [Feeder]: ../../feeders/index.html
    /// [views::Multiselect]: ../../views/struct.Multiselect.html
    /// [with_factory_view]: ../multiselect/struct.MultiselectManager.html#method.with_factory_view
    pub fn with_feeder<T: Feeder>(feeder: T) -> Self {
        MultiselectManager {
            feeder: Rc::new(feeder),
            view_factory: None,
        }
    }
    /// Creates an instance with customized [views::Multiselect].
    ///
    /// If you want to specify only a [Feeder] (and use a default [views::Multiselect])
    /// use [with_feeder]
    ///
    /// [Feeder]: ../../feeders/index.html
    /// [views::Multiselect]: ../../views/struct.Multiselect.html
    /// [with_feeder]: ../multiselect/struct.MultiselectManager.html#method.with_feeder
    pub fn with_factory_view(factory: Rc<Fn() -> views::Multiselect>) -> Self {
        MultiselectManager {
            // it should be an option of Rc :)
            feeder: Rc::new(DummyFeeder),
            view_factory: Some(factory),
        }
    }
    fn get_view(&self) -> views::Multiselect {
        let view = if let Some(ref fun) = self.view_factory {
            fun()
        } else {
            views::Multiselect::new(Rc::clone(&self.feeder))
        };
        view
    }
}

impl WidgetManager for MultiselectManager {
    fn build_value_view(&self, initial: &str) -> ViewBox {
        let mut widget = self.get_view();
        if initial.trim() != "" {
            let items = initial
                .split(VALUE_SEP)
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            widget.select_items(items);
        }
        ViewBox::new(Box::new(widget))
    }
    fn build_widget(&self, label: &str, help: &str, initial: &str) -> ViewBox {
        let view = self.build_value_view(initial);
        label_with_help_layout(view, label, help)
    }
    fn get_value(&self, view_box: &ViewBox) -> String {
        let view_box = fields::value_view_from_layout(view_box);
        let ms: &views::Multiselect = (**view_box).as_any().downcast_ref().unwrap();

        let result: Vec<String> = ms
            .get_selected_items()
            .iter()
            .map(|x| (*x).to_owned())
            .collect();
        result.join(VALUE_SEP)
    }
}

impl FormField for Field<MultiselectManager, Vec<String>> {
    fn get_widget_manager(&self) -> &WidgetManager {
        &self.widget_manager
    }
    fn validate(&self, data: &str) -> Result<Value, FieldErrors> {
        let mut errors = FieldErrors::new();
        let items = data.split(VALUE_SEP).collect::<Vec<&str>>();
        for item in items.iter() {
            for v in &self.validators {
                if let Some(e) = v.validate(item) {
                    errors.push(e);
                }
            }
        }
        if errors.len() > 0 {
            Err(errors)
        } else {
            let vec_str = items
                .iter()
                .map(|x| Value::String(x.to_string()))
                .collect::<Vec<Value>>();
            let val_of_vec = Value::Array(vec_str);
            Ok(val_of_vec)
        }
    }
    fn get_label(&self) -> &str {
        &self.label
    }

    /// Gets help of the field
    fn get_help(&self) -> &str {
        self.help.as_ref()
    }

    fn get_initial(&self) -> String {
        self.initial.join(VALUE_SEP)
    }

    fn clap_arg(&self) -> clap::Arg {
        clap::Arg::with_name(&self.label)
            .long(&self.label)
            .help(&self.help)
            .required(self.is_required())
            .multiple(true)
            .takes_value(true)
    }

    fn clap_args2str(&self, args: &clap::ArgMatches) -> String {
        let values = args
            .values_of(&self.label)
            .unwrap_or(clap::Values::default());
        values.collect::<Vec<&str>>().join(VALUE_SEP)
    }
    fn is_required(&self) -> bool {
        self.is_required()
    }
}

impl<W: WidgetManager> Field<W, Vec<String>> {
    /// Sets initial `value` of `field`.
    pub fn initial<U: Deref<Target = str>>(mut self, initial: Vec<U>) -> Self {
        self.initial = initial
            .iter()
            .map(|x| (*x).to_string())
            .collect::<Vec<String>>();
        self
    }
}
