use std::rc::Rc;

use clap;
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
    //TODO::: rm it
    //) -> fields::Field<AutocompleteManager, String> {
    ) -> Field2 {
        //TODO::: rm it
        //fields::Field::new(label, AutocompleteManager(Rc::new(feeder)), "".to_string())
        Field2::new(AutocompleteManager(Rc::new(feeder)))
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

    // NEW API

    fn take_view(&mut self) -> ViewBox {
        ViewBox::new(Box::new(DummyView))
    }
}



//TODO::: cleanups
use cursive::view::ViewWrapper;
use cursive::views::{LinearLayout, TextView, DummyView};
use validators::{Required, Validator};
//TODO::: rename to Field/Autocomplete/or whatever
/// TODO:::
pub struct Field2 {
    view: LinearLayout,
    widget_manager: AutocompleteManager,
}
impl Field2 {
    fn new(mut widget_manager: AutocompleteManager) -> Field2 {
        let layout = LinearLayout::vertical()
                    //TODO:: label can't include separator
                    .child(TextView::new(""))
                    .child(widget_manager.take_view())
                    .child(TextView::new(""))
                    .child(DummyView);
        Field2 {
            view: layout,
            widget_manager: widget_manager,
        }
    }
    // COMPAT STUFF
    //TODO::: use it
    pub fn initial<IS: Into<String>>(mut self, initial: IS) -> Self {
        /// Sets initial `value` of `field`.
        //self.initial = initial.into();
        self
    }
    /// Sets `help` message for `field`.
    pub fn help<IS: Into<String>>(mut self, msg: IS) -> Self {
        //TODO:::
        //self.help = msg.into();
        self
    }
    /// Append `validator`.
    pub fn validator<V: Validator + 'static>(mut self, validator: V) -> Self {
        //TODO:::
        //self.validators.push(Rc::new(validator));
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
}
impl fields::FormField for Field2 {
    fn get_widget_manager(&self) -> &WidgetManager {
        //TODO::: cleanups
        &self.widget_manager
    }
    fn build_widget(&self) -> ViewBox {
        //TODO::: cleanups
        self.widget_manager
            .build_widget("", "", "")
    }
    fn validate(&self, data: &str) -> Result<Value, FieldErrors> {
        //TODO::: cleanups
        let mut errors = FieldErrors::new();
        //for v in &self.validators {
        //    if let Some(e) = v.validate(data) {
        //        errors.push(e);
        //    }
        //}
        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(Value::String(data.to_owned()))
        }
    }

    /// Gets label of the field
    fn get_label(&self) -> &str {
        //TODO::: cleanups
        //&self.label
        ""
    }

    fn clap_arg(&self) -> clap::Arg {
        //TODO::: cleanups
        clap::Arg::with_name("")
            //.help(&self.help)
            //.long(&self.label)
            //.required(self.is_required())
            //.takes_value(true)
    }

    fn clap_args2str(&self, args: &clap::ArgMatches) -> String {
        //TODO::: cleanups
        args.value_of("").unwrap_or("").to_string()
    }
}
impl ViewWrapper for Field2 {
    wrap_impl!(self.view: LinearLayout);
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
}

//TODO::: rm it
impl<W: WidgetManager> fields::Field<W, String> {
    /// Sets initial `value` of `field`.
    pub fn initial<IS: Into<String>>(mut self, initial: IS) -> Self {
        self.initial = initial.into();
        self
    }
}
