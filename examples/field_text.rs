// Demonstrates features of field Text
extern crate cursive;
extern crate fui;
extern crate regex;
extern crate serde_json;

use cursive::traits::Boxable;
use cursive::views::Dialog;
use cursive::Cursive;
use regex::Regex;
use serde_json::value::Value;

use fui::fields::Text;
use fui::form::FormView;
use fui::validators::Required;

fn show_data(c: &mut Cursive, data: Value) {
    let text = format!("Got data: {:?}", data);
    c.add_layer(Dialog::info(text));
}

fn main() {
    let mut siv = Cursive::default();

    let form = FormView::new()
        .field(Text::new("basic-field"))
        .field(Text::new("help-for-field").help("help message"))
        .field(Text::new("initialized-field").initial("some-value"))
        .field(Text::new("with-validator").validator(Required))
        .field(
            Text::new("with-validators")
                .validator(Required)
                .validator(Regex::new("[0-9]").unwrap()),
        )
        .field(
            Text::new("all-in-one")
                .help("help")
                .initial("some text")
                .validator(Required),
        )
        .on_submit(show_data);
    siv.add_layer(Dialog::around(form).fixed_width(50));

    siv.run();
}
