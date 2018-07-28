// Demonstrates features of field Multiselect
extern crate cursive;
extern crate fui;
extern crate serde_json;

use cursive::traits::Boxable;
use cursive::views::Dialog;
use cursive::Cursive;
use serde_json::value::Value;

use fui::fields::Multiselect;
use fui::form::FormView;
use fui::validators::{OneOf, Required};

fn show_data(c: &mut Cursive, data: Value) {
    let text = format!("Got data: {:?}", data);
    c.add_layer(Dialog::info(text));
}

fn main() {
    let mut siv = Cursive::default();

    let options = vec!["option1", "option2", "option3", ".."];

    let form = FormView::new()
        .field(Multiselect::new("basic-field", options.clone()))
        .field(Multiselect::new("field-with-help", options.clone()).help("help message"))
        .field(Multiselect::new("initialized-field", options.clone()).initial(options.clone()))
        .field(Multiselect::new("with-validator", options.clone()).validator(Required))
        .field(
            Multiselect::new("with-validators", options.clone()).validator(OneOf(options.clone())),
        )
        .field(
            Multiselect::new("all-in-one", options.clone())
                .help("help")
                .initial(options.clone())
                .validator(Required),
        )
        .on_submit(show_data);
    siv.add_layer(Dialog::around(form).full_screen());

    siv.run();
}
