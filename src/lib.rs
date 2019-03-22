//! `fui` add [CLI] and text form interface to a program.
//!
//! [CLI]: https://en.wikipedia.org/wiki/Command-line_interface
//!
//! ## Examples
//!
//! ### Cargo.toml
//! ```toml, no_run
//! [dependencies]
//! fui = "0.9"
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
//!     Fui::new("program_name")
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
//! * [Fields]
//!
//! These components will be most frequently used building blocks, especially [FormView] and
//! [Fields].
//!
//! [Fui]: struct.Fui.html
//! [FormView]: form/struct.FormView.html
//! [Fields]: fields/index.html
//!
//! Here's the logic behind those components:
//!
//! * [Fui] is a struct which gathers your program `action`s
//! * `action`s are things which program does (like, `git pull`, `git push`, etc.)
//! * `action` includes:
//!     * description: this should shortly explain to `user` what an `action` does
//!     * [FormView]: is a container for [Fields]
//!         * [Fields] represents data used during `action` execution
//!     * handler: is a `fn`/`closure` called after user fills the `Form`
//!
//!
//! ### Flow:
//!
//! 1) user picks `action` (then `form` is shown)
//! 2) user submit `form` with `data`
//! 3) `handler` is called with `data`
//!
#![deny(missing_docs)]

extern crate clap;
extern crate clipboard;
#[macro_use]
extern crate cursive as _cursive;
extern crate dirs;
extern crate glob;
extern crate regex;
extern crate serde_json;

// TODO: make it public when ready
mod clap_conv;

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

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use cursive::event::Event;
use cursive::traits::{Boxable, Identifiable};
use cursive::views::{Dialog, LayerPosition, OnEventView};
use cursive::Cursive;
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

fn value2array(value: &Value) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if let Value::Object(map) = value {
        for (key, val) in map {
            match val {
                Value::Bool(true) => {
                    result.push(format!("--{}", key));
                },
                Value::Number(n) => {
                    result.push(format!("--{}", key));
                    result.push(format!("{}", n));
                },
                Value::String(s) => {
                    result.push(format!("--{}", key));
                    result.push(format!("{}", s));
                },
                Value::Array(vals) => {
                    result.push(format!("--{}", key));
                    for v in vals {
                        if v.is_string() {
                            result.push(v.as_str().unwrap().into());
                        }
                    }
                },
                Value::Object(_) => {
                    result.push(format!("{}", key));
                    let mut found = value2array(&val);
                    result.append(&mut found);
                }
                _ => (),
            };
        }
    }
    result
}

trait DumpAsCli {
    fn dump_as_cli(&self) -> String;
}

impl DumpAsCli for Value {
    fn dump_as_cli(&self) -> String {
        if self.is_object() {
            let cmd = self
                .as_object()
                .unwrap()
                .iter()
                .map({
                    |(k, v)| match *v {
                        Value::Bool(true) => format!("--{}", k),
                        Value::String(ref s) => format!("--{} \"{}\"", k, s),
                        Value::Number(ref n) => format!("--{} {}", k, n),
                        Value::Array(ref v) => {
                            let args = v
                                .iter()
                                .map(|vv| format!("{}", vv))
                                .collect::<Vec<String>>()
                                .join(" ");
                            format!("--{} {}", k, args)
                        }
                        _ => "".to_string(),
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");
            cmd
        } else {
            "".to_string()
        }
    }
}

/// Top level building block of `fui` crate.
pub struct Fui<'attrs, 'action> {
    actions: BTreeMap<String, Action<'action>>,
    name: &'attrs str,
    version: &'attrs str,
    about: &'attrs str,
    author: &'attrs str,
    theme: &'attrs str,
    picked_action: Rc<RefCell<Option<String>>>,
    form_data: Rc<RefCell<Option<Value>>>,
}
impl<'attrs, 'action> Fui<'attrs, 'action> {
    /// Creates a new `Fui` with empty actions.
    pub fn new(program_name: &'attrs str) -> Self {
        Fui {
            actions: BTreeMap::new(),
            name: program_name,
            version: "",
            about: "",
            author: "",
            theme: &DEFAULT_THEME,
            picked_action: Rc::new(RefCell::new(None)),
            form_data: Rc::new(RefCell::new(None)),
        }
    }
    /// Defines action by providing `name`, `help`, `form`, `hdlr`.
    ///
    /// NOTE:
    ///
    /// `name` is also translated into CLI argument, so:
    ///
    /// * "my-arg" is ok (only `"a..z"` & `"-"`)
    /// * "my arg" is bad (becuase in shell space (`" "`) needs to be escaped)
    ///
    /// # Panics:
    ///
    /// Panics if action name is duplicated.
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

        if let Some(item) = self.action_by_name(&name) {
            panic!(
                "Action name must be unique, but it's already defined ({:?})",
                item.cmd_with_desc()
            );
        }
        self.actions
            .insert(action_details.cmd_with_desc(), action_details);
        self
    }

    fn action_by_name(&self, name: &str) -> Option<&Action> {
        self.actions.values().find(|a| a.name == name)
    }

    fn actions(&self) -> Vec<&Action> {
        self.actions.values().collect()
    }

    /// Coordinates flow from action picking to handler running.
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

    fn dump_as_cli(&self) -> Vec<String> {
        let mut arg_vec = vec![self.name.to_owned()];
        if let Some(a) = self.picked_action.borrow().as_ref() {
            if *a != arg_vec[0] {
                arg_vec.push(a.to_owned())
            }
        }
        if let Some(f) = self.form_data.borrow().as_ref() {
            arg_vec.append(&mut value2array(&f));
        }
        arg_vec
    }

    fn set_action(&mut self, name: &str) {
        if let Some(a) = self.action_by_name(name) {
            *self.picked_action.borrow_mut() = Some(a.name.to_string());
        }
    }

    fn set_form_data(&mut self, form_data: Value) {
        *self.form_data.borrow_mut() = Some(form_data);
    }

    /// Gets user input converted to cli-like format
    pub fn get_cli_input(mut self) -> Vec<String> {
        self.input_from_tui();
        self.dump_as_cli()
    }



    /// Returns automatiacally generated [clap::App].
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
        let action = self
            .actions
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

    fn set_form_events(&self, form: &mut FormView) {
        // set form events
        let form_data = Rc::clone(&self.form_data);
        form.set_on_submit(move |c: &mut Cursive, data: Value| {
            *form_data.borrow_mut() = Some(data);
            c.quit();
        });
        form.set_on_cancel(move |c: &mut Cursive| {
            c.screen_mut().move_to_front(LayerPosition::FromFront(1))
        });
    }

    fn add_form(&self, c: &mut Cursive, form: FormView, form_id: &str) {
        // `with_id` must be before `OnEventView`
        let form = form.with_id(form_id).full_width();
        let prog_name = self.name.to_owned();
        let form_id = form_id.to_owned();
        let form = OnEventView::new(form).on_event(Event::CtrlChar('k'), move |c| {
            let err = c.call_on_id(&form_id, |form: &mut FormView| match form.validate() {
                Ok(s) => {
                    let msg = format!("{} {} {}", prog_name, form_id, s.dump_as_cli());
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    ctx.set_contents(msg).unwrap();
                    Ok(())
                }
                Err(_) => {
                    let err = format!("Copying to clipboard - FAILED.\nFix form errors first.");
                    Err(err)
                }
            });
            if let Err(e) = err.unwrap() {
                c.add_layer(Dialog::info(e).title("Form invalid!"));
            }
        });
        c.add_layer(form);
    }

    fn add_forms(&mut self, c: &mut Cursive) {
        // seperate loop prevents borrow-mut (self.actions) & borrow (any method call)
        let action_form_list = self
            .actions
            .iter_mut()
            .map(|(_, a)| (a.name, a.form.take().unwrap()))
            .collect::<Vec<(&str, FormView)>>();
        for (form_id, mut form) in action_form_list.into_iter() {
            self.set_form_events(&mut form);
            self.add_form(c, form, form_id);
        }
    }

    fn add_cmd_picker(&mut self, c: &mut Cursive) {
        let cmd_submit = Rc::clone(&self.picked_action);
        let cmd_cancel = Rc::clone(&self.picked_action);
        // TODO: rm cloning for it
        let actions = self
            .actions
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
                    // here we return name+desc of Action, we can't return only name
                    // because Action has shorter lifetime then this callback (which is static)
                    // so thanks to lifetime, they saved me a bug :)
                    *cmd_submit.borrow_mut() = Some(value.as_str().unwrap().to_string());
                    c.quit();
                })
                .on_cancel(move |c| {
                    *cmd_cancel.borrow_mut() = None;
                    c.quit()
                })
                .full_screen(),
        );
    }

    fn top_form_by_action_name(&self, cursive: &mut Cursive, action_name: &str) {
        let stack = cursive.screen_mut();
        let from = stack.find_layer_from_id(action_name).unwrap();
        stack.move_layer(from, LayerPosition::FromFront(0));
    }

    fn input_from_tui(&mut self) -> Option<(String, Value)> {
        // Cursive blocks stdout, unless it's dropped, so
        // deattached cursive here to allow destroying it at the end of this fn
        let mut c = Cursive::default();
        c.load_toml(self.theme).expect("Can't load theme");
        self.add_forms(&mut c);
        self.add_cmd_picker(&mut c);

        let mut action_name = "";
        while *self.form_data.borrow() == None {
            c.run();
            let action_with_desc = match self.picked_action.borrow().clone() {
                Some(v) => v,
                None => return None,
            };
            // to get action name we have to extract it from "name: desc"
            action_name = self.actions.get(&action_with_desc).unwrap().name;
            self.top_form_by_action_name(&mut c, action_name);
        }
        *self.picked_action.borrow_mut() = Some(action_name.to_string());
        Some((
            self.picked_action.borrow().clone().unwrap(),
            self.form_data.borrow().clone().unwrap(),
        ))
    }

    /// Sets program's `name`.
    ///
    /// For CLI means [Clap::App::name]
    ///
    /// [clap::App::name]: ../clap/struct.App.html#method.name
    pub fn name(mut self, name: &'attrs str) -> Self {
        self.name = name;
        self
    }

    /// Gets program's `name`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Gets program's `about`.
    pub fn get_about(&self) -> &str {
        &self.about
    }

    /// Gets program's `author`.
    pub fn get_author(&self) -> &str {
        &self.author
    }

    /// Gets program's `version`.
    pub fn get_version(&self) -> &str {
        &self.version
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
    /// let app = Fui::new("program_name")
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
mod test_date_getting_from_program_args {
    use super::*;

    #[test]
    fn cli_checkbox_is_serialized_ok_when_value_preset() {
        let value = Fui::new("app")
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
        let value = Fui::new("app")
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
        let value = Fui::new("app")
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
        let value = Fui::new("app")
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
        let value = Fui::new("app")
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

#[cfg(test)]
mod test_dumping_value_to_cli_command {
    use super::*;

    #[test]
    fn test_value_is_converted_to_cmd_ok_when_is_string() {
        let v: Value = serde_json::from_str(r#"{ "arg": "abc" }"#).unwrap();
        assert_eq!(v.dump_as_cli(), r#"--arg "abc""#);
    }

    #[test]
    fn test_value_is_converted_to_cmd_ok_when_is_array() {
        let v: Value = serde_json::from_str(r#"{ "arg": ["a", "b", "c"] }"#).unwrap();
        assert_eq!(v.dump_as_cli(), r#"--arg "a" "b" "c""#);
    }

    #[test]
    fn test_value_is_empty_when_arg_is_false() {
        let v: Value = serde_json::from_str(r#"{ "arg": false }"#).unwrap();
        assert_eq!(v.dump_as_cli(), r#""#);
    }
}

#[cfg(test)]
mod cli_args_tese {
    use super::*;

    #[test]
    fn test_value_empty_object_is_converted_to_empty_array() {
        let v: Value = serde_json::from_str(r#"{}"#).unwrap();
        let found: Vec<String> = value2array(&v);
        let expected: Vec<String> = Vec::new();
        assert_eq!(found, expected);
    }

    #[test]
    fn test_value_object_with_bool_false_is_converted_to_empty_array() {
        let v: Value = serde_json::from_str(r#"{"arg": false}"#).unwrap();
        let found: Vec<String> = value2array(&v);
        let expected: Vec<String> = Vec::new();
        assert_eq!(found, expected);
    }

    #[test]
    fn test_value_object_with_bool_true_is_converted_to_arg() {
        let v: Value = serde_json::from_str(r#"{"arg": true}"#).unwrap();
        let found: Vec<String> = value2array(&v);
        let expected: Vec<String> = vec!["--arg".to_string()];
        assert_eq!(found, expected);
    }

    #[test]
    fn test_value_object_with_numerical_is_converted_to_arg() {
        let v: Value = serde_json::from_str(r#"{"arg": 5}"#).unwrap();
        let found: Vec<String> = value2array(&v);
        let expected: Vec<String> = vec!["--arg".to_string(), "5".to_string()];
        assert_eq!(found, expected);
    }

    #[test]
    fn test_value_object_with_string_is_converted_to_arg() {
        let v: Value = serde_json::from_str(r#"{"arg": "text"}"#).unwrap();
        let found: Vec<String> = value2array(&v);
        let expected: Vec<String> = vec!["--arg".to_string(), "text".to_string()];
        assert_eq!(found, expected);
    }

    #[test]
    fn test_value_object_with_array_is_converted_to_arg() {
        let v: Value = serde_json::from_str(r#"{"arg": ["a", "b", "c"]}"#).unwrap();
        let found: Vec<String> = value2array(&v);
        let expected: Vec<String> = vec![
            "--arg", "a", "b", "c"
        ].iter().map(|x| x.to_string()).collect();
        assert_eq!(found, expected);
    }

    #[test]
    fn test_value_object_with_empty_object_is_converted_to_arg() {
        let v: Value = serde_json::from_str(
            r#"{"subcmd": {}}"#
        ).unwrap();
        let found: Vec<String> = value2array(&v);
        let expected: Vec<String> = vec!["subcmd".into()];
        assert_eq!(found, expected);
    }

    #[test]
    fn test_value_object_with_text_in_object_is_converted_to_arg() {
        let v: Value = serde_json::from_str(
            r#"{"subcmd": {"arg": "text"}}"#
        ).unwrap();
        let found: Vec<String> = value2array(&v);
        let expected: Vec<String> = vec![
            "subcmd", "--arg", "text",
        ].iter().map(|x| x.to_string()).collect();
        assert_eq!(found, expected);
    }
}
