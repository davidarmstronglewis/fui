//! `fui` lets you build a form based user interfaces for a [CLI] program.
//!
//! [CLI]: https://en.wikipedia.org/wiki/Command-line_interface
//!
//! ## Examples
//!
//! ### Cargo.toml
//! ```toml, no_run
//! [dependencies]
//! fui = "0.7"
//! ```
//!
//! ### main.rs
//! ```no_run
//! extern crate fui;
//! ```
//!
//!
//!
//! ```no_run
//! extern crate fui;
//!
//! use fui::{Fui, Value};
//! use fui::form::FormView;
//! use fui::fields::Text;
//!
//! fn hdlr(v: Value) {
//!     println!("user input (from hdlr) {:?}", v);
//! }
//!
//! fn main() {
//!     Fui::new()
//!         .action(
//!             "ACTION1: description",
//!             FormView::new().field(Text::new("action1 data").help("help for action1 data")),
//!             |v| {
//!                 println!("user input (from callback) {:?}", v);
//!             },
//!         )
//!         .action(
//!             "ACTION2: description",
//!             FormView::new().field(Text::new("action2 data").help("help for action2 data")),
//!             hdlr,
//!         )
//!         .run();
//! }
//! ```
//! <div>
//! <details> <summary>Click me to see screen</summary>
//! <a href="https://raw.githubusercontent.com/xliiv/fui/master/doc/app_basic.png">
//! <img src="https://raw.githubusercontent.com/xliiv/fui/master/doc/app_basic.png" alt="app_basic.rs example", width="100%" />
//! </a>
//! </details>
//! </div>
//!
//! ### More examples
//!
//! <div>
//! <ul>
//!
//! <li>
//! <a href="https://github.com/xliiv/fui/blob/master/examples/app_ln_like.rs">app_ln_like</a>
//! <details> <summary>Click me to see screen</summary>
//! <a href="https://raw.githubusercontent.com/xliiv/fui/master/doc/app_ln_like.png">
//! <img src="https://raw.githubusercontent.com/xliiv/fui/master/doc/app_ln_like.png" alt="app_ln_like.rs example", width="100%" />
//! </a>
//! </details>
//! </li>
//!
//! <li>
//! <a href="https://github.com/xliiv/fui/blob/master/examples/app_tar_like.rs">app_tar_like</a>
//! <details> <summary>Click me to see screen</summary>
//! <a href="https://raw.githubusercontent.com/xliiv/fui/master/doc/app_tar_like.png">
//! <img src="https://raw.githubusercontent.com/xliiv/fui/master/doc/app_tar_like.png" alt="app_tar_like.rs example", width="100%" />
//! </a>
//! </details>
//! </li>
//!
//! </ul>
//! </div>
//!
//!
//! ## Description
//!
//! If you look at the example above you'll notice a few entities:
//!
//! * [Fui]
//! * [FormView]
//! * [fields]
//!
//! These components will be most frequently used building blocks, especially [FormView] and
//! [fields].
//!
//! [Fui]: struct.Fui.html
//! [FormView]: form/struct.FormView.html
//! [fields]: fields/index.html
//!
//! Here's the logic behind those components:
//!
//! * [Fui] is a struct which gathers your program `action`s
//! * `action`s are things which program does (like, `git pull`, `git push`, etc.)
//! * `action` includes:
//!     * description: this should shortly explain to `user` what `action` does
//!     * [FormView]: is a container for [fields]
//!         * [fields] represents data used during `action` execution
//!     * handler: is a `fn`/`callback` called after user fills the `Form`
//!
//!
//! ### Flow:
//!
//! 1) user picks `action` (then `form` is shown)
//! 2) user submit `form` with `data`
//! 3) `handler` is called with `data` (point 2)
//!
#![deny(missing_docs)]

extern crate clap;
#[macro_use]
extern crate cursive as _cursive;
extern crate glob;
extern crate regex;
extern crate serde_json;

/// Re-export of [Cursive](../cursive/index.html) crate.
pub mod cursive {
    pub use _cursive::*;
}
pub use serde_json::value::Value;
pub mod feeders;
pub mod fields;
pub mod form;
pub mod utils;
pub mod validators;
pub mod views;

use cursive::Cursive;
use cursive::traits::Boxable;
use form::FormView;
use std::cell::RefCell;
use std::env;
use std::ffi::OsString;
use std::rc::Rc;
use validators::OneOf;

/// Top level building block of `fui` crate
pub struct Fui {
    //TODO:: no Strings
    actions: Vec<(String, String)>,
    // TODO:: new structure should allows:
    // action-name should be stored as lower-case and upper-case only in picker
    // action-name 2 form
    // action-name 2 desc
    // action-name 2 handler
    descs: Vec<String>,

    forms: Vec<FormView>,
    hdlrs: Vec<Box<Fn(Value) + 'static>>,
}
impl Fui {
    /// Creates a new `Fui` with empty actions
    pub fn new() -> Self {
        Fui {
            actions: Vec::new(),
            descs: Vec::new(),
            forms: Vec::new(),
            hdlrs: Vec::new(),
        }
    }
    /// Defines action by providing `desc`, `form`, `hdlr`
    pub fn action<IS, F>(mut self, desc: IS, form: FormView, hdlr: F) -> Self
    where
        IS: Into<String>,
        F: Fn(Value) + 'static,
    {
        let desc = desc.into();

        {
            //TODO:: char validation for action-name
            let action_data: Vec<&str> = desc.splitn(2, ": ").collect();
            if action_data.len() == 1 {
                self.actions
                    .push((action_data[0].to_string(), "".to_string()));
            } else {
                self.actions
                    .push((action_data[0].to_string(), action_data[1].to_string()));
            };
        }

        //TODO:: validate desc includes at least on :
        //let data = {
        //    "archive-files": {
        //        desc: "",
        //        form_idx: usize,
        //        handler: rc<fn>
        //    }
        //}
        self.descs.push(desc);
        self.forms.push(form);
        self.hdlrs.push(Box::new(hdlr));
        self
    }

    /// Coordinates flow from action picking to handler running
    pub fn run(mut self) {
        let args = env::args_os();
        if args.len() > 1 {
            let (value, selected_idx) = self.run_cli(args);
            // TODO:: refactor it with cli version below
            let hdlr = self.hdlrs.remove(selected_idx);
            hdlr(value)
        } else {
            // input from TUI
            let (form_data, selected_idx) = {
                // cursive instance breaks println!, enclose it with scope to fix printing
                let mut c = cursive::Cursive::new();

                // cmd picker
                let mut cmd: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
                let cmd_clone = Rc::clone(&cmd);
                c.add_layer(
                    FormView::new()
                        .field(
                            fields::Autocomplete::new("action", self.descs.clone())
                                .help("Pick action")
                                .validator(OneOf(self.descs.clone())),
                        )
                        .on_submit(move |c, data| {
                            let value = data.get("action").unwrap().clone();
                            *cmd_clone.borrow_mut() = Some(value.as_str().unwrap().to_string());
                            c.quit();
                        })
                        .on_cancel(|c| c.quit())
                        .full_screen(),
                );
                c.run();
                let selected_idx = cmd.borrow()
                    .clone()
                    .and_then(|v| self.descs.iter().position(|item| item == v.as_str()));
                let selected_idx = match selected_idx {
                    None => return,
                    Some(idx) => idx,
                };

                // form
                let mut form_view = self.forms.remove(selected_idx);
                let mut form_data: Rc<RefCell<Option<Value>>> = Rc::new(RefCell::new(None));
                let mut form_data_submit = Rc::clone(&form_data);
                form_view.set_on_submit(move |c: &mut Cursive, data: Value| {
                    *form_data_submit.borrow_mut() = Some(data);
                    c.quit();
                });
                form_view.set_on_cancel(move |c: &mut Cursive| {
                    //TODO: this should return to action picker
                    //TODO: self.forms are drained so can't be done now
                    c.quit();
                });
                c.add_layer(form_view.full_width());
                c.run();
                (form_data, selected_idx)
            };

            // run handler
            let form_data = Rc::try_unwrap(form_data).unwrap().into_inner();
            if let Some(data) = form_data {
                let hdlr = self.hdlrs.remove(selected_idx);
                hdlr(data)
            }
        }
    }

    fn run_cli<I, T>(&self, user_args: I) -> (Value, usize)
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let user_args = user_args
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<OsString>>();

        let mut sub_cmds: Vec<clap::App> = Vec::new();
        for (idx, form) in self.forms.iter().enumerate() {
            let args = form.fields2clap_args();
            let (ref name, ref help) = self.actions[idx];
            let sub_cmd = clap::SubCommand::with_name(&name)
                .about(help.as_ref())
                .args(args.as_slice());
            sub_cmds.push(sub_cmd);
        }
        let app = clap::App::new(user_args[0].as_os_str().to_str().unwrap())
            // TODO:: .version(version)
            // TODO:: .author(author)
            // TODO:: .about(about)
            .subcommands(sub_cmds);
        let matches = app.get_matches_from(user_args);
        let cmd_name = matches.subcommand_name().unwrap();
        let selected_idx = self.actions.iter().position(|x| x.0 == cmd_name).unwrap();
        let cmd_matches = matches.subcommand_matches(cmd_name).unwrap();
        let idx = self.actions.iter().position(|x| x.0 == cmd_name).unwrap();
        let form = &self.forms[idx];
        let value = form.clap_arg_matches2value(cmd_matches);
        (value, selected_idx)
    }

    //TODO::
    //fn run_tui(mut self) -> () {
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_checkbox_is_serialized_ok_when_value_preset() {
        let value = Fui::new()
            .action(
                "action1: desc",
                FormView::new().field(fields::Checkbox::new("ch1")),
                |_| {},
            )
            .run_cli(vec!["my_app", "action1", "--ch1"]);

        let exp: Value = serde_json::from_str(r#"{ "ch1": true }"#).unwrap();
        assert_eq!(value.0, exp);
    }

    #[test]
    fn cli_checkbox_is_serialized_ok_when_value_missing() {
        let value = Fui::new()
            .action(
                "action1: desc",
                FormView::new().field(fields::Checkbox::new("ch1")),
                |_| {},
            )
            .run_cli(vec!["my_app", "action1"]);

        let exp: Value = serde_json::from_str(r#"{ "ch1": false }"#).unwrap();
        assert_eq!(value.0, exp);
    }

    #[test]
    fn cli_text_is_serialized_ok_when_value_preset() {
        let value = Fui::new()
            .action(
                "action1: desc",
                FormView::new().field(fields::Text::new("t1")),
                |_| {},
            )
            .run_cli(vec!["my_app", "action1", "--t1", "v1"]);

        let exp: Value = serde_json::from_str(r#"{ "t1": "v1" }"#).unwrap();
        assert_eq!(value.0, exp);
    }

    //#[test]
    //fn cli_text_is_serialized_ok_when_value_missing() {
    //    // clap blocks this case, optionally test ensuring that
    //}

    #[test]
    fn cli_autocomplete_is_serialized_ok_when_value_preset() {
        let value = Fui::new()
            .action(
                "action1: desc",
                FormView::new().field(fields::Autocomplete::new("ac", vec!["v1", "v2", "v3"])),
                |_| {},
            )
            .run_cli(vec!["my_app", "action1", "--ac", "v1"]);

        let exp: Value = serde_json::from_str(r#"{ "ac": "v1" }"#).unwrap();
        assert_eq!(value.0, exp);
    }

    //#[test]
    //fn cli_autocomplete_is_serialized_ok_when_value_missing() {
    //    // clap blocks this case, optionally test ensuring that
    //}

    #[test]
    fn cli_multiselect_is_serialized_ok_when_value_preset() {
        let value = Fui::new()
            .action(
                "action1: desc",
                FormView::new().field(fields::Multiselect::new("mf", vec!["v1", "v2", "v3"])),
                |_| {},
            )
            .run_cli(vec!["my_app", "action1", "--mf", "v1"]);
        let exp: Value = serde_json::from_str(r#"{ "mf": ["v1"] }"#).unwrap();
        assert_eq!(value.0, exp);
    }

    //#[test]
    //fn cli_multiselect_is_serialized_ok_when_value_missing() {
    //    // clap blocks this case, optionally test ensuring that
    //}
}
