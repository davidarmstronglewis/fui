// EXPERIMENTAL - no guarantee of finishing it

extern crate clap;
extern crate fui;

use fui::Fui;
use clap::{App, Arg};
use std::env;

fn main() {
    let app = App::new("virtua_fighter")
        .arg(
            Arg::with_name("some-switch")
                .long("arg-long")
                .help("arg-help")
        )
        ;

    let mut _arg_vec: Vec<String> = env::args().collect();
    if _arg_vec.len() <= 1 {
        _arg_vec = Fui::from(&app).get_cli_input();
    }

    println!("args {:?}", &_arg_vec);
    let matches = app.get_matches_from(_arg_vec);
    println!("{:?}", matches);
}
