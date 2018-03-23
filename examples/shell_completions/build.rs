#[macro_use]
extern crate clap;
extern crate fui;

use clap::Shell;

include!("src/app.rs");

fn main() {
    let app = build_app();
    let mut cli = app.build_cli_app();
    cli.gen_completions("clap_completition", // We need to specify the bin name manually
                        Shell::Bash,         // Then say which shell to build completions for
                        ".");                // Then say where write the completions to
}
