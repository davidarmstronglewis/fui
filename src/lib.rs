//! `fui` lets you build a form based user interfaces for a [CLI] program.
//!
//! [CLI]: https://en.wikipedia.org/wiki/Command-line_interface
//!
//! ## Examples
//!
//! ### Cargo.toml
//! ```toml, no_run
//! [dependencies]
//! fui = "0.8"
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
//!     println!("user input (from fn) {:?}", v);
//! }
//!
//! fn main() {
//!     Fui::new()
//!         .action(
//!             "action1",
//!             "description",
//!             FormView::new().field(Text::new("action1 data").help("help for action1 data")),
//!             |v| {
//!                 println!("user input (from closure) {:?}", v);
//!             },
//!         )
//!         .action(
//!             "action2",
//!             "description",
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
use std::collections::BTreeMap;
use std::env;
use std::ffi::OsString;
use std::rc::Rc;
use validators::OneOf;

const DEFAULT_THEME: &'static str = "
[colors]
    highlight_inactive = \"light black\"
";

struct Action<'action> {
    name: &'action str,
    help: &'action str,
    form: Option<FormView>,
    handler: Rc<Fn(Value)>,
}

impl<'action> Action<'action> {
    fn cmd_with_desc(&self) -> String {
        format!("{}: {}", self.name, self.help)
    }
}

/// Top level building block of `fui` crate
pub struct Fui<'attrs, 'action> {
    actions: BTreeMap<String, Action<'action>>,
    name: &'attrs str,
    version: &'attrs str,
    about: &'attrs str,
    author: &'attrs str,
    theme: &'attrs str,
}
impl<'attrs, 'action> Fui<'attrs, 'action> {
    /// Creates a new `Fui` with empty actions
    pub fn new() -> Self {
        Fui {
            actions: BTreeMap::new(),
            name: "",
            version: "",
            about: "",
            author: "",
            theme: &DEFAULT_THEME,
        }
    }
    /// Defines action by providing `name`, `help`, `form`, `hdlr`
    ///
    /// NOTE:
    ///
    /// `name` is also translated into CLI argument, so:
    ///
    /// * "my-arg" is ok (only `"a..z"` & `"-"`)
    /// * "my arg" is bad (becuase in shell space (`" "`) needs to be escaped)
    ///
    pub fn action<F>(
        mut self,
        name: &'action str,
        help: &'action str,
        form: FormView,
        hdlr: F,
    ) -> Self
    where
        F: Fn(Value) + 'static,
    {
        let action_details = Action {
            name: name,
            help: help,
            form: Some(form),
            handler: Rc::new(hdlr),
        };
        self.actions
            .insert(action_details.cmd_with_desc(), action_details);
        self
    }

    /// Coordinates flow from action picking to handler running
    // This must be moving, until FormView implements copy or FormViews are added to cursive once
    // then top layer are switched (instead of current inserting/popping)
    pub fn run(mut self) {
        let args = env::args_os();
        let input_data = if args.len() > 1 {
            // input from CLI
            self.input_from_cli(args)
        } else {
            // input from TUI
            self.input_from_tui()
        };
        // run handler
        if let Some((action, data)) = input_data {
            let hdlr = self.actions.get(&action).unwrap().handler.clone();
            hdlr(data);
        }
    }

    /// Returns automatiacally generated [clap::App]
    ///
    /// [clap::App]: ../clap/struct.App.html
    pub fn build_cli_app(&self) -> clap::App {
        let mut sub_cmds: Vec<clap::App> = Vec::new();
        for action in self.actions.values() {
            let args = action.form.as_ref().unwrap().fields2clap_args();
            let sub_cmd = clap::SubCommand::with_name(action.name.as_ref())
                .about(action.help.as_ref())
                .args(args.as_slice());
            sub_cmds.push(sub_cmd);
        }
        clap::App::new(self.name.as_ref())
            .version(self.version.as_ref())
            .about(self.about.as_ref())
            .author(self.author.as_ref())
            .subcommands(sub_cmds)
    }

    fn input_from_cli<I, T>(&self, user_args: I) -> Option<(String, Value)>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let user_args = user_args
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<OsString>>();

        let app = self.build_cli_app();

        let matches = app.get_matches_from(user_args);
        let cmd_name = matches.subcommand_name().unwrap();
        let cmd_matches = matches.subcommand_matches(cmd_name).unwrap();
        let action = self.actions
            .values()
            .find(|action| action.name == cmd_name)
            .unwrap();
        let value = action
            .form
            .as_ref()
            .unwrap()
            .clap_arg_matches2value(cmd_matches);
        Some((action.cmd_with_desc(), value))
    }

    fn header(&self) -> String {
        let header = if (self.name.len() > 0) & (self.version.len() > 0) {
            format!("{} ({})", self.name, self.version)
        } else if self.name.len() > 0 {
            format!("{}", self.name)
        } else {
            format!("")
        };
        return header;
    }

    fn run_tui_cmd_picker(&self, c: &mut Cursive) -> Rc<RefCell<Option<String>>> {
        let cmd: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        let cmd_clone = Rc::clone(&cmd);
        // TODO: rm cloning for it
        let actions = self.actions
            .keys()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        c.add_layer(
            FormView::new()
                .title(&self.header())
                .field(
                    fields::Autocomplete::new("action", actions.clone())
                        .help("Pick action")
                        .validator(OneOf(actions)),
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
        cmd
    }

    fn input_from_tui(&mut self) -> Option<(String, Value)> {
        let mut c = cursive::Cursive::new();
        c.load_theme(self.theme).expect("Can't load theme");

        let cmd = self.run_tui_cmd_picker(&mut c);
        let selection = match cmd.borrow().clone() {
            Some(v) => v,
            None => return None,
        };

        // form
        // TODO: use find_layer_from_id when available
        // https://github.com/
        // gyscos/Cursive/commit/06305c89a9223ffa0b041c94df4a51a177b1c99a
        // #diff-bbe86c39b8f295bd78f682413bd99e5aR247
        let action = self.actions.get_mut(&selection).unwrap();
        let mut form_view = (*action).form.take().unwrap();

        let form_data: Rc<RefCell<Option<Value>>> = Rc::new(RefCell::new(None));
        let form_data_submit = Rc::clone(&form_data);
        form_view.set_on_submit(move |c: &mut Cursive, data: Value| {
            *form_data_submit.borrow_mut() = Some(data);
            c.quit();
        });
        form_view.set_on_cancel(move |c: &mut Cursive| {
            //TODO: this should return to action picker
            //TODO: forms are drained so can't be done now
            c.quit();
        });
        c.add_layer(form_view.full_width());
        c.run();
        let form_data = form_data.borrow().clone().unwrap();
        Some((selection, form_data))
    }

    /// Sets program's `name.
    ///
    /// For CLI means [Clap::App::name]
    ///
    /// [clap::App::name]: ../clap/struct.App.html#method.name
    pub fn name(mut self, name: &'attrs str) -> Self {
        self.name = name;
        self
    }

    /// Sets program's `version`.
    ///
    /// For CLI means [Clap::App::version]
    ///
    /// [clap::App::version]: ../clap/struct.App.html#method.version
    pub fn version(mut self, version: &'attrs str) -> Self {
        self.version = version;
        self
    }

    /// Sets program's `about`.
    ///
    /// For CLI means [Clap::App::about]
    ///
    /// [clap::App::about]: ../clap/struct.App.html#method.about
    pub fn about(mut self, about: &'attrs str) -> Self {
        self.about = about;
        self
    }

    /// Sets program's `author`.
    ///
    /// For CLI means [Clap::App::author]
    ///
    /// [clap::App::author]: ../clap/struct.App.html#method.author
    pub fn author(mut self, author: &'attrs str) -> Self {
        self.author = author;
        self
    }

    /// Sets `theme` for `Fui`.
    ///
    /// For details see [Cursive's themes]
    ///
    /// # Example:
    ///
    /// ```
    /// use fui::Fui;
    /// use fui::form::FormView;
    /// use fui::fields::Text;
    ///
    /// let my_theme = "
    /// shadow = false
    /// borders = \"simple\"
    /// [colors]
    ///     background = \"yellow\"
    /// ";
    ///
    /// let app = Fui::new()
    ///     .action(
    ///         "action1",
    ///         "desc",
    ///         FormView::new().field(Text::new("action1-data")),
    ///         |v| { println!("{:?}", v); }
    ///     )
    ///     .theme(my_theme);
    /// ```
    ///
    ///
    /// [Cursive's themes]: ../cursive/theme/index.html#themes
    pub fn theme(mut self, theme: &'attrs str) -> Self {
        self.theme = theme;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_checkbox_is_serialized_ok_when_value_preset() {
        let value = Fui::new()
            .action(
                "action1",
                "desc",
                FormView::new().field(fields::Checkbox::new("ch1")),
                |_| {},
            )
            .input_from_cli(vec!["my_app", "action1", "--ch1"]);

        let exp: Value = serde_json::from_str(r#"{ "ch1": true }"#).unwrap();
        assert_eq!(value, Some(("action1: desc".to_string(), exp)));
    }

    #[test]
    fn cli_checkbox_is_serialized_ok_when_value_missing() {
        let value = Fui::new()
            .action(
                "action1",
                "desc",
                FormView::new().field(fields::Checkbox::new("ch1")),
                |_| {},
            )
            .input_from_cli(vec!["my_app", "action1"]);

        let exp: Value = serde_json::from_str(r#"{ "ch1": false }"#).unwrap();
        assert_eq!(value, Some(("action1: desc".to_string(), exp)));
    }

    #[test]
    fn cli_text_is_serialized_ok_when_value_preset() {
        let value = Fui::new()
            .action(
                "action1",
                "desc",
                FormView::new().field(fields::Text::new("t1")),
                |_| {},
            )
            .input_from_cli(vec!["my_app", "action1", "--t1", "v1"]);

        let exp: Value = serde_json::from_str(r#"{ "t1": "v1" }"#).unwrap();
        assert_eq!(value, Some(("action1: desc".to_string(), exp)));
    }

    //#[test]
    //fn cli_text_is_serialized_ok_when_value_missing() {
    //    // clap blocks this case, optionally test ensuring that
    //}

    #[test]
    fn cli_autocomplete_is_serialized_ok_when_value_preset() {
        let value = Fui::new()
            .action(
                "action1",
                "desc",
                FormView::new().field(fields::Autocomplete::new("ac", vec!["v1", "v2", "v3"])),
                |_| {},
            )
            .input_from_cli(vec!["my_app", "action1", "--ac", "v1"]);

        let exp: Value = serde_json::from_str(r#"{ "ac": "v1" }"#).unwrap();
        assert_eq!(value, Some(("action1: desc".to_string(), exp)));
    }

    //#[test]
    //fn cli_autocomplete_is_serialized_ok_when_value_missing() {
    //    // clap blocks this case, optionally test ensuring that
    //}

    #[test]
    fn cli_multiselect_is_serialized_ok_when_value_preset() {
        let value = Fui::new()
            .action(
                "action1",
                "desc",
                FormView::new().field(fields::Multiselect::new("mf", vec!["v1", "v2", "v3"])),
                |_| {},
            )
            .input_from_cli(vec!["my_app", "action1", "--mf", "v1"]);
        let exp: Value = serde_json::from_str(r#"{ "mf": ["v1"] }"#).unwrap();
        assert_eq!(value, Some(("action1: desc".to_string(), exp)));
    }

    //#[test]
    //fn cli_multiselect_is_serialized_ok_when_value_missing() {
    //    // clap blocks this case, optionally test ensuring that
    //}
}
