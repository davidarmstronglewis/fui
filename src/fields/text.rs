use clap;
use cursive::views;
use serde_json::value::Value;

use fields;
use fields::{FieldErrors, WidgetManager};

/// Convienient wrapper around `Field<TextManager, String>`.
pub struct Text;

impl Text {
    /// Creates a new `Field<TextManager, String>`.
    pub fn new<IS: Into<String>>(label: IS) -> fields::Field<TextManager, String> {
        fields::Field::new(label, TextManager, "".to_string())
    }
}

#[derive(Clone)]
pub struct TextManager;

impl WidgetManager for TextManager {
    fn build_widget(&self, label: &str, help: &str, initial: &str) -> views::ViewBox {
        let view = self.build_value_view(initial);
        fields::label_with_help_layout(view, label, help)
    }
    fn get_value(&self, view_box: &views::ViewBox) -> String {
        let view_box = fields::value_view_from_layout(view_box);
        let edit: &views::EditView = (**view_box).as_any().downcast_ref().unwrap();
        let value: String = (&*edit.get_content()).clone();
        value
    }
    fn build_value_view(&self, value: &str) -> views::ViewBox {
        views::ViewBox::new(Box::new(views::EditView::new().content(value)))
    }
}

impl fields::FormField for fields::Field<TextManager, String> {
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
            .long(&self.label)
            .help(&self.help)
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

impl<W: WidgetManager> fields::Field<W, String> {
    /// Sets initial `value` of `field`.
    pub fn initial<IS: Into<String>>(mut self, initial: IS) -> Self {
        self.initial = initial.into();
        self
    }
}
