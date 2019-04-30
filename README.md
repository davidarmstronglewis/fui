# fui


[![docs.rs](https://docs.rs/fui/badge.svg)](https://docs.rs/crate/fui)
[![crates.io](https://meritbadge.herokuapp.com/fui)](https://crates.io/crates/fui)
[![Build Status](https://travis-ci.org/xliiv/fui.svg?branch=master)](https://travis-ci.org/xliiv/fui)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


Add CLI & form interface to your program.

## Basic example

### Cargo.toml
```toml
[dependencies]
fui = "0.9"
```

### main.rs
```rust
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
    Fui::new(crate_name!())
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
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .run();
}
```

This will make the program automatically working in 2 modes:

1. Ready for parsing CLI arguments, like here:

    ```bash
    $ ./app_basic -h
    app_basic 1.0.0
    xliiv <tymoteusz.jankowski@gmail.com>
    An Example program which has CLI & form interface (TUI)

    USAGE:
        app_basic [SUBCOMMAND]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    SUBCOMMANDS:
        action1    help for action1
        action2    help for action2
        help       Prints this message or the help of the given subcommand(s)
    ```

2. Ready for getting user input from easy and discoverable TUI interface, like image below:


## More examples

[Here](https://github.com/xliiv/fui/tree/master/examples)


## Screens

<a href="https://github.com/xliiv/fui/blob/master/examples/app_basic.rs">
<img src="https://raw.githubusercontent.com/xliiv/fui/master/doc/app_basic.png" alt="app_basic.rs example", width="100%" />
</a>


<a href="https://github.com/xliiv/fui/blob/master/examples/app_ln_like.rs">
<img src="https://raw.githubusercontent.com/xliiv/fui/master/doc/app_ln_like.png" alt="app_ln_like.rs example", width="100%" />
</a>

<a href="https://github.com/xliiv/fui/blob/master/examples/app_tar_like.rs">
<img src="https://raw.githubusercontent.com/xliiv/fui/master/doc/app_tar_like.png" alt="app_tar_like.rs example", width="100%" />
</a>


## TODO
* find a solution for long help messages
* ctrl+enter submits ([#151](https://github.com/gyscos/Cursive/issues/151))
* handle unwraps

## Ideas
* `.validator(OneOf || Regex::new("v\d+\.\d+\.\d+")).unwrap()`?
* support user's history?
* checkboxes: automatic toggle on char(+alt)?
* replace `views::Autocomplete` & `views::Multiselect` with a new implementation of
  `Autocomplete`
