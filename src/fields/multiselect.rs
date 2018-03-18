use std::ops::Deref;
use std::rc::Rc;

use clap;
use cursive::view::AnyView;
use cursive::views::{LinearLayout, TextView};
use serde_json::value::Value;

use feeders::Feeder;
use fields::{label_with_help_layout, Field, FormField, WidgetManager};
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
        let mngr = MultiselectManager {
            feeder: Rc::new(feeder),
        };
        Field::new(label, mngr, Vec::new())
    }
}

#[derive(Clone)]
pub struct MultiselectManager {
    feeder: Rc<Feeder>,
}

impl WidgetManager for MultiselectManager {
    fn build_value_view(&self, initial: &str) -> Box<AnyView> {
        let mut widget = views::Multiselect::new(Rc::clone(&self.feeder));
        if initial.trim() != "" {
            let items = initial
                .split(VALUE_SEP)
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            widget.select_items(items);
        }
        Box::new(widget)
    }
    fn build_widget(&self, label: &str, help: &str, initial: &str) -> Box<AnyView> {
        let view = self.build_value_view(initial);
        label_with_help_layout(view, label, help)
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
        let ms = (**boxed_field)
            .as_any()
            .downcast_ref::<views::Multiselect>()
            .unwrap();

        let result: Vec<String> = ms.get_selected_items()
            .iter()
            .map(|x| (*x).to_owned())
            .collect();
        result.join(VALUE_SEP)
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
}

impl FormField for Field<MultiselectManager, Vec<String>> {
    fn get_widget_manager(&self) -> &WidgetManager {
        &self.widget_manager
    }
    fn validate(&self, data: &str) -> Result<Value, String> {
        let items = data.split(VALUE_SEP).collect::<Vec<&str>>();
        for item in items.iter() {
            for v in &self.validators {
                if let Some(e) = v.validate(item) {
                    return Err(e);
                }
            }
        }
        let vec_str = items
            .iter()
            .map(|x| Value::String(x.to_string()))
            .collect::<Vec<Value>>();
        let val_of_vec = Value::Array(vec_str);
        Ok(val_of_vec)
    }
    fn get_label(&self) -> &str {
        &self.label
    }
    fn build_widget(&self) -> Box<AnyView> {
        let initial = self.initial.join(VALUE_SEP);
        self.widget_manager
            .build_widget(&self.label, &self.help, &initial)
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
        let values = args.values_of(&self.label)
            .unwrap_or(clap::Values::default());
        values.collect::<Vec<&str>>().join(VALUE_SEP)
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
