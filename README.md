# fui


[![docs.rs](https://docs.rs/fui/badge.svg)](https://docs.rs/crate/fui)
[![crates.io](https://meritbadge.herokuapp.com/fui)](https://crates.io/crates/fui)
[![Build Status](https://travis-ci.org/xliiv/fui.svg?branch=master)](https://travis-ci.org/xliiv/fui)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


Add CLI & form interface to your program.


## Basic example

### cargo.toml

```toml
[dependencies]
fui = "0.9"
```

#### Using with [`clap`](https://crates.io/crates/clap) (experimental)

```rust
extern crate clap;
extern crate fui;

use clap::{App, Arg};
use fui::Fui;
use std::env;

// regular clap code
let app = App::new("some-app").arg(
    Arg::with_name("some-switch")
        .long("arg-long")
        .help("arg-help"),
);


// extra fui code
let mut _arg_vec: Vec<String> = env::args().collect();
if _arg_vec.len() <= 1 {
    _arg_vec = Fui::from(&app).get_cli_input();
}


// regular clap code
let matches = app.get_matches_from(_arg_vec);
```

<a href="https://github.com/xliiv/fui/blob/master/examples/clap-flags.rs">
<img src="https://raw.githubusercontent.com/xliiv/fui/master/doc/clap-flags-example.gif" alt="clap to fui flags example", width="100%" />
</a>

#### Using without [`clap`](https://crates.io/crates/clap)

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


## Clap support

### Implemented features
* switch arguments
* positional arguments
* option arguments
* global arguments
* subcommands (single level)

### To be implemented
* [conflicts_with](https://docs.rs/clap/2.33.0/clap/struct.Arg.html#method.conflicts_with)
* [requires](https://docs.rs/clap/2.33.0/clap/struct.Arg.html#method.requires)
* [validators](https://docs.rs/clap/2.33.0/clap/struct.Arg.html#method.validator)
* [min](https://docs.rs/clap/2.33.0/clap/struct.Arg.html#method.min_values)/[max](https://docs.rs/clap/2.33.0/clap/struct.Arg.html#method.max_values)/[exact](https://docs.rs/clap/2.33.0/clap/struct.Arg.html#method.number_of_values) number of values for
    * positional args
    * options args
* [groups](https://docs.rs/clap/2.33.0/clap/struct.Arg.html#method.group)
* [conditional defaults](https://docs.rs/clap/2.33.0/clap/struct.Arg.html#method.default_value_if)
* [custom delimeter](https://docs.rs/clap/2.33.0/clap/struct.Arg.html#method.use_delimiter)


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
