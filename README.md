# fui


[![docs.rs](https://docs.rs/fui/badge.svg)](https://docs.rs/crate/fui)
[![crates.io](https://meritbadge.herokuapp.com/fui)](https://crates.io/crates/fui)
[![Build Status](https://travis-ci.org/xliiv/fui.svg?branch=master)](https://travis-ci.org/xliiv/fui)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


Add CLI & form interface to your program.

**Note: Use it at own risk!!**

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

    or

    ```bash
    $ ./app_basic action1 -h
    app_basic-action1 
    help for action1

    USAGE:
        app_basic action1 [OPTIONS]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
            --action1-data <action1-data>    help for action1 data
    ```

2. Ready for getting user input from easy and discoverable TUI interface, like image below:


### More examples

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


## TODO:

* empty forms are skipped and executed
* .validator(OneOf || Regex::new("v\d+\.\d+\.\d+")).unwrap()
    * or something similiar
* add option for prose help in widget?
* add option for prose help in cmd-picker?
    * "path" in "rustup toolchain link -h"
    * perhaps adding new cmd is walkaround, like "link-help" which displays prose
* reusing fields in form by cloning is stupid
* command picker show only 5 items Oo
* expose view's options (eg. submit_anything) on Autocomplete & Multiselect
* reusing fields in each form, like "verbose for each form"
* allow disabling copying (ctrl+k)


* support user's history!
    * make fill-error-correct flow pleasent
* support for piping!
* create wrapper FileField
* create wrapper DirField
* ctrl+enter submits ([#151](https://github.com/gyscos/Cursive/issues/151))?
* checkbox: automatic toggle on char
* add Field.data & form on_submit returns it?
* optimalizations
    * feeders use iterators
    * thread
* more tests
* error handling & unwraps
* magic stuff:
    * ~~add magic which renders form for clap (or structopt) if args missing~~
        * `clap` should distinguish data types: file, dir, other
    * add magic which works with current programs like: ls, grep, etc.
