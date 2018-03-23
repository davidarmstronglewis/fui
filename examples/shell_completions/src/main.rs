#[macro_use]
extern crate clap;
extern crate fui;

mod app;

use app::build_app;

fn main() {
    let app = build_app();
    app.run();
}
