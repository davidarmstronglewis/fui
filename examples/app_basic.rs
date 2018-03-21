// Example showing imagined CLI app. with two actions

#[macro_use]
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
        // optional attrs, however it's recommended to set them otherwise "" will be used
        // here values are taken from Cargo.toml (see: clap doc https://docs.rs/clap/)
        // NOTE:
        // Running this example will show program name as "fui" (instead of "app_basic")
        // because that's the value in Cargo.toml (similar to rest of attrs)
        .name(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .run();
}
