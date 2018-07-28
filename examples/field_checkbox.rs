// Demonstrates features of field Checkbox
extern crate cursive;
extern crate fui;
extern crate serde_json;

use cursive::views::Dialog;
use cursive::Cursive;
use serde_json::value::Value;

use fui::fields::Checkbox;
use fui::form::FormView;

fn show_data(c: &mut Cursive, data: Value) {
    let text = format!("Got data: {:?}", data);
    c.add_layer(Dialog::info(text));
}

fn main() {
    let mut siv = Cursive::default();

    let form = FormView::new()
        .field(Checkbox::new("basic-field"))
        .field(Checkbox::new("help-for-field").help("help message"))
        .field(Checkbox::new("checked-field").initial(true))
        .field(Checkbox::new("all-in-one").help("help").initial(true))
        .on_submit(show_data);
    siv.add_layer(Dialog::around(form));

    siv.run();
}
