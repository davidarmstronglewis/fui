// EXPERIMENTAL - no guarantee of finishing it

extern crate clap;
extern crate fui;

use clap::{App, Arg};
use fui::Fui;
use std::env;

fn main() {
    let app = App::new("virtua_fighter")
        .arg(Arg::with_name("pos").help("help").index(1))
        .arg(
            Arg::with_name("pos-default")
                .help("pos-default-help")
                .index(2)
                .default_value("default-value"),
        )
        .arg(
            Arg::with_name("pos-multi")
                .help("pos-multi-help")
                .index(3)
                .multiple(true),
        )
        .arg(
            Arg::with_name("pos-default-multi")
                .help("pos-default-multi-help")
                .index(4)
                .multiple(true)
                .default_value("default1 default2"),
        );

    let mut _arg_vec: Vec<String> = env::args().collect();
    if _arg_vec.len() <= 1 {
        _arg_vec = Fui::from(&app).get_cli_input();
    }

    println!("args {:?}", &_arg_vec);
    let matches = app.get_matches_from(_arg_vec);
    println!("{:?}", matches);
}
