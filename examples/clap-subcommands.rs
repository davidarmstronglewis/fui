// Shows conversion from `clap` including `subcommands` to `fui`

extern crate clap;
extern crate fui;

use fui::Fui;
use std::env;

fn main() {
    let app = clap::App::new("some-app")
        .subcommand(
            clap::SubCommand::with_name("simple-subcmd").about("Does something subcommandish"),
        )
        .subcommand(
            clap::SubCommand::with_name("subcmd-with-arg")
                .about("Does something subcommandish but with arg")
                .arg(
                    clap::Arg::with_name("subcmd-arg-name")
                        .long("subcmd-arg-long")
                        .help("Help for subcmd-arg"),
                ),
        );

    let mut _arg_vec: Vec<String> = env::args().collect();
    if _arg_vec.len() <= 1 {
        _arg_vec = Fui::from(&app).get_cli_input();
    }

    println!("args {:?}", &_arg_vec);
    let matches = app.get_matches_from(_arg_vec);
    println!("{:?}", matches);
}
