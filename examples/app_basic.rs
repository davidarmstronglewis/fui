// Example showing imagined CLI app. with two actions

extern crate clap;
extern crate fui;

use fui::{Fui, Value};
use fui::form::FormView;
use fui::fields::Text;

fn hdlr(v: Value) {
    println!("user input (from fn) {:?}", v);
}

fn main() {
    Fui::new()
        .action(
            "action1",
            "help for action1",
            FormView::new().field(Text::new("action1-data").help("help for action1 data")),
            |v| {
                println!("user input (from closure) {:?}", v);
            },
        )
        .action(
            "action2",
            "help for action2",
            FormView::new().field(Text::new("action2-data").help("help for action2 data")),
            hdlr,
        )
        .about("An Example program which has CLI & form interface (TUI)")
        .run();
}
