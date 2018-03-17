use std::str::FromStr;

use clap;
use cursive::view::AnyView;
use cursive::views;
use serde_json::value::Value;

use fields;
use fields::WidgetManager;

/// Convienient wrapper around `Field<CheckboxManager, bool>`.
pub struct Checkbox;

impl Checkbox {
    /// Creates a new `Field<CheckboxManager, bool>`.
    pub fn new<IS: Into<String>>(label: IS) -> fields::Field<CheckboxManager, bool> {
        fields::Field::new(label, CheckboxManager, false)
    }
}

#[derive(Clone)]
pub struct CheckboxManager;

impl fields::WidgetManager for CheckboxManager {
    fn build_widget(&self, label: &str, help: &str, initial: &str) -> Box<AnyView> {
        let checkbox = self.build_value_view(&initial);
        fields::label_with_help_layout(checkbox, &label, &help)
    }
    fn get_value(&self, view: &AnyView) -> String {
        let boxed_widget = view.as_any().downcast_ref::<Box<AnyView>>().unwrap();
        let widget = (**boxed_widget)
            .as_any()
            .downcast_ref::<views::LinearLayout>()
            .unwrap();
        let boxed_field = widget
            .get_child(1)
            .unwrap()
            .as_any()
            .downcast_ref::<Box<AnyView>>()
            .unwrap();
        let checkbox = (**boxed_field)
            .as_any()
            .downcast_ref::<views::Checkbox>()
            .unwrap();
        let value = checkbox.is_checked();
        format!("{}", value)
    }
    fn set_error(&self, _view: &mut AnyView, _error: &str) {
        // no operation, checkbox is always valid
    }
    fn build_value_view(&self, value: &str) -> Box<AnyView> {
        let value = FromStr::from_str(value).unwrap();
        let mut checkbox = views::Checkbox::new();
        checkbox.set_checked(value);
        Box::new(checkbox)
    }
}

impl fields::FormField for fields::Field<CheckboxManager, bool> {
    fn get_widget_manager(&self) -> &WidgetManager {
        &self.widget_manager
    }
    fn build_widget(&self) -> Box<AnyView> {
        let initial = format!("{}", self.initial);
        self.widget_manager
            .build_widget(&self.label, &self.help, &initial)
    }
    fn get_label(&self) -> &str {
        &self.label
    }
    fn validate(&self, data: &str) -> Result<Value, String> {
        let value = FromStr::from_str(data)
            .map(|v| Value::Bool(v))
            .map_err(|_| "Value can't be converterd to bool".to_string());
        value
    }

    fn clap_arg(&self) -> clap::Arg {
        clap::Arg::with_name(&self.label)
            .help(&self.help)
            .long(&self.label)
    }

    fn clap_args2str(&self, args: &clap::ArgMatches) -> String {
        let v = if args.is_present(&self.label) {
            "true"
        } else {
            "false"
        };
        v.to_string()
    }
}

impl<W: WidgetManager> fields::Field<W, bool> {
    /// Sets initial `value` of `field`.
    pub fn initial(mut self, value: bool) -> Self {
        self.initial = value;
        self
    }
}
