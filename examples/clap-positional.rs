// EXPERIMENTAL - no guarantee of finishing it

extern crate clap;
extern crate fui;

use clap::{App, Arg};
use fui::Fui;
use std::env;

fn main() {
    let app = App::new("virtua_fighter")
        .arg(
            Arg::with_name("pos")
                .index(1)
                .help("help")
                .default_value("a")
                .possible_values(&["a", "a a", "a a a"]),
        )
        .arg(
            Arg::with_name("pos-multi")
                .index(2)
                .help("pos-multi-help")
                .default_value("b")
                .possible_values(&["b", "b b", "b b b"])
                .multiple(true),
        );

    let mut _arg_vec: Vec<String> = env::args().collect();
    if _arg_vec.len() <= 1 {
        _arg_vec = Fui::from(&app).get_cli_input();
    }

    println!("args {:?}", &_arg_vec);
    let matches = app.get_matches_from(_arg_vec);
    println!("{:?}", matches);
}
