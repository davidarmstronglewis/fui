// Demonstrates available form fields
extern crate cursive;
extern crate fui;
extern crate serde_json;

use cursive::traits::Boxable;
use cursive::views::Dialog;
use cursive::Cursive;
use serde_json::value::Value;

use fui::fields::{Autocomplete, Checkbox, Multiselect, Text};
use fui::form::FormView;

fn submit_handler(c: &mut Cursive, data: Value) {
    let text = format!("submit data: {:?}", data);
    c.add_layer(Dialog::info(text));
}

fn main() {
    let mut siv = Cursive::default();

    let options = vec!["op1", "op2", "op3"];

    let form = FormView::new()
        .field(Checkbox::new("verbose").help("this is help for checkbox"))
        .field(Text::new("text-field").help("this is help for text"))
        .field(
            Autocomplete::new("autocomplete-field", options.clone())
                .help("this is help for autocomplete"),
        )
        .field(
            Multiselect::new("multiselect-field", options.clone())
                .help("this is help for multiselect"),
        )
        .on_submit(submit_handler)
        .on_cancel(|c| c.quit());

    siv.add_layer(form.full_screen());

    siv.run();
}
