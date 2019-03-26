// EXPERIMENTAL - no guarantee of finishing it

extern crate clap;
extern crate fui;

use clap::{App, Arg};
use fui::Fui;
use std::env;

fn main() {
    let app = App::new("virtua_fighter")
        .arg(
            Arg::with_name("option")
                .takes_value(true)
                .long("option-long")
                .help("option-help"),
        )
        .arg(
            Arg::with_name("option-multi")
                .takes_value(true)
                .long("option-multi-long")
                .help("option-multi-help")
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
