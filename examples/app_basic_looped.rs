// Example showing imagined CLI app. with two looped actions

extern crate fui;

use std::io;

use fui::fields::Text;
use fui::form::FormView;
use fui::{Fui, Value};

fn hdlr(v: Value) {
    println!("user input (from fn) {:?}", v);
}

fn main() {
    loop {
        Fui::new("app_basic_looped")
            .action(
                "action1",
                "description",
                FormView::new().field(Text::new("action1 data").help("help for action1 data")),
                |v| {
                    println!("user input (from closure) {:?}", v);
                },
            )
            .action(
                "action2",
                "description",
                FormView::new().field(Text::new("action2 data").help("help for action2 data")),
                hdlr,
            )
            .run();

        println!("\nContinue? [Y,n]");
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "n" {
            break;
        }
    }
}
