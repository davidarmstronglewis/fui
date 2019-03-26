// Demonstrates features of field Text
extern crate cursive;
extern crate fui;
extern crate serde_json;

use cursive::traits::Boxable;
use cursive::views::Dialog;
use cursive::Cursive;
use serde_json::value::Value;

use fui::fields::Autocomplete;
use fui::form::FormView;
use fui::validators::{OneOf, Required};

fn show_data(c: &mut Cursive, data: Value) {
    let text = format!("Got data: {:?}", data);
    c.add_layer(Dialog::info(text));
}

fn main() {
    let mut siv = Cursive::default();

    // see examples/feeders.rs for more completion options
    let options = vec!["op1", "op2", "op3"];

    let form = FormView::new()
        .field(Autocomplete::new("basic-field", options.clone()))
        .field(Autocomplete::new("field-with-help", options.clone()).help("help message"))
        .field(Autocomplete::new("initialized-field", options.clone()).initial("3"))
        .field(Autocomplete::new("with-validator", options.clone()).validator(Required))
        .field(
            Autocomplete::new("with-validators", options.clone()).validator(OneOf(options.clone())),
        )
        .field(
            Autocomplete::new("all-in-one", options.clone())
                .help("help")
                .initial("some text")
                .validator(Required),
        )
        .on_submit(show_data);
    siv.add_layer(Dialog::around(form).full_screen());

    siv.run();
}
