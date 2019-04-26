//!
//! This is an EXPERIMENTAL feature on very early stage.
//!

use clap;
use clap::ArgSettings;
use feeders::DirItems;
use fields::autocomplete::AutocompleteManager;
use fields::multiselect::MultiselectManager;
use fields::{Autocomplete, Checkbox, Field, FormField, Text};
use form::FormView;
use std::ffi::OsStr;
use validators::Required;
use std::rc::Rc;
use views;
use Fui;

fn show_warn(msg: &'static str) {
    // TODO: find a better way for warning users
    // crate log requires to use env var to make messages visible
    // so we need something better
    panic!(msg);
}

/// Splits `value` on `delimeter`.
fn split_values(value: &str, delimeter: char) -> Vec<String> {
    let new_delimeter = if value.contains("\"") {
        format!("\"{}", delimeter)
    } else {
        delimeter.to_string()
    };
    let found = value
        .split(new_delimeter.as_str())
        .map(|i| i.trim_matches('"').to_string())
        .collect();
    found
}

/// Copies `default` value to `field`.
fn copy_default(
    mut field: Field<AutocompleteManager, String>,
    default: Option<&OsStr>,
) -> Field<AutocompleteManager, String> {
    if let Some(v) = default {
        if let Some(s) = v.to_str() {
            field = field.initial::<String>(s.to_string());
        }
    }
    field
}

/// Copies multi `default` value to `field`.
fn copy_default_multi(
    mut field: Field<MultiselectManager, Vec<String>>,
    default: Option<&OsStr>,
    delimeter: Option<char>,
) -> Field<MultiselectManager, Vec<String>> {
    if let Some(v) = default {
        if let Some(s) = v.to_str() {
            let delimeter = delimeter.unwrap_or(' ');
            let values = split_values(s, delimeter);
            field = field.initial::<String>(values);
        }
    }
    field
}

/// Gets field depends on `values`.
fn field_with_vals<V: Into<String>>(
    values: &Option<Vec<&str>>,
    name: V,
    help: &str,
) -> Field<AutocompleteManager, String> {
    if let Some(ref vals) = values {
        let options = vals.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        Autocomplete::new(name.into(), options).help(help)
    } else {
        Autocomplete::new(name, DirItems::new()).help(help)
    }
}

/// Gets multi field depends on `values`.
fn field_multi_with_vals<V: Into<String>>(
    values: &Option<Vec<&str>>,
    name: V,
    help: &str,
) -> Field<MultiselectManager, Vec<String>> {
    let mngr = if let Some(ref vals) = values {
        let options = vals.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        MultiselectManager::with_factory_view(
            Rc::new(move || {
                views::Multiselect::new(options.clone()).select_anything().redundant_selection()
            })
        )
    } else {
        MultiselectManager::with_factory_view(
            Rc::new(move || {
                views::Multiselect::new(DirItems::new()).select_anything().redundant_selection()
            })
        )
    };
    Field::new(name, mngr, Vec::new()).help(help)
}

fn clap_app2fields(clap_app: &clap::App) -> Vec<Box<FormField>> {
    let mut field_list = Vec::new();
    // TODO: flag & option & positional loops are mostly copy & paste so make it DRY
    // using AnyArg can help, see
    // https://github.com/clap-rs/clap/blob/9d31e63b28eff81ad35239268a38ce3b2d2d635d/src/args/any_arg.rs#L12
    for (idx, pos) in clap_app.p.positionals.iter() {
        //println!("POSITIONAL {:?} {:?}\n", idx, pos.b);
        if pos.b.blacklist.is_some() {
            show_warn("Args dependency (via `clap::Arg::conflicts_with`) is not supported yet");
        }
        if pos.b.requires.is_some() {
            show_warn("Args dependency (via `clap::Arg::requires`) is not supported yet");
        }
        // TODO: improve by allowing short + help?
        // TODO: add attr. shown_field_name and use pos.b.name for it
        // because now integers are shown as field name
        let long = format!("{}", idx);
        let help = pos
            .b
            .help
            .expect(&format!("Arg {:?} must have help", pos.b.name));
        if pos.b.settings.is_set(ArgSettings::Multiple) {
            let mut field = field_multi_with_vals(&pos.v.possible_vals, long, help);
            if pos.b.settings.is_set(ArgSettings::Required) {
                field = field.validator(Required);
            }
            field = copy_default_multi(field, pos.v.default_val, pos.v.val_delim);
            field_list.push(Box::new(field) as Box<FormField>);
        } else {
            let mut field = field_with_vals(&pos.v.possible_vals, long, help);
            field = copy_default(field, pos.v.default_val);
            if pos.b.settings.is_set(ArgSettings::Required) {
                field = field.validator(Required);
            }
            field_list.push(Box::new(field) as Box<FormField>);
        }
    }
    for option in clap_app.p.opts.iter() {
        //println!("OPTION {:?}\n", option.b);
        if option.b.blacklist.is_some() {
            show_warn("Args dependency (via `clap::Arg::conflicts_with`) is not supported yet");
        }
        if option.b.requires.is_some() {
            show_warn("Args dependency (via `clap::Arg::requires`) is not supported yet");
        }
        // TODO: improve by allowing short + help?
        let long = option
            .s
            .long
            .expect(&format!("Arg {:?} must have long name", option.b.name));
        let help = option
            .b
            .help
            .expect(&format!("Arg {:?} must have help", option.b.name));
        if option.b.settings.is_set(ArgSettings::Multiple) {
            let mut field = field_multi_with_vals(&option.v.possible_vals, long, help);
            if option.b.settings.is_set(ArgSettings::Required) {
                field = field.validator(Required);
            }
            field = copy_default_multi(field, option.v.default_val, option.v.val_delim);
            field_list.push(Box::new(field) as Box<FormField>);
        } else {
            let mut field = field_with_vals(&option.v.possible_vals, long, help);
            if option.b.settings.is_set(ArgSettings::Required) {
                field = field.validator(Required);
            }
            field = copy_default(field, option.v.default_val);
            field_list.push(Box::new(field) as Box<FormField>);
        }
    }
    for flag in clap_app.p.flags.iter() {
        //println!("FLAG {:?}\n", flag.b);
        if flag.b.blacklist.is_some() {
            show_warn("Args dependency (via `clap::Arg::conflicts_with`) is not supported yet");
        }
        if flag.b.requires.is_some() {
            show_warn("Args dependency (via `clap::Arg::requires`) is not supported yet");
        }
        // TODO: improve by allowing short + help?
        let long = flag
            .s
            .long
            .expect(&format!("Arg {:?} must have long name", flag.b.name));
        let help = flag
            .b
            .help
            .expect(&format!("Arg {:?} must have help", flag.b.name));
        if flag.b.settings.is_set(ArgSettings::Multiple) {
            // TODO: add validator for a positive integer
            let field = Text::new(long).help(help);
            field_list.push(Box::new(field) as Box<FormField>);
        } else {
            let field = Checkbox::new(long).help(help);
            field_list.push(Box::new(field) as Box<FormField>);
        }
    }
    field_list
}

fn copy_clap_fields_to_form<'a>(clap_app: &'a clap::App, mut form: FormView) -> FormView {
    let mut fields = clap_app2fields(clap_app);
    for field in fields.drain(..) {
        form = form.boxed_field(field);
    }
    form
}

impl<'a> From<&'a clap::App<'_, '_>> for FormView {
    fn from(clap_app: &'a clap::App) -> Self {
        let mut form = FormView::new();
        form = copy_clap_fields_to_form(clap_app, form);
        form
    }
}

impl<'a> From<&'a clap::App<'_, '_>> for Fui<'a, 'a> {
    fn from(clap_app: &'a clap::App) -> Self {
        let mut fui = Fui::new(clap_app.get_name())
            .about(clap_app.p.meta.about.unwrap_or(""))
            .author(clap_app.p.meta.author.unwrap_or(""))
            .version(clap_app.p.meta.version.unwrap_or(""))
            .skip_single_action(true)
            .skip_empty_form(true);

        //println!("{:?}", clap_app.p.flags);

        if clap_app.p.subcommands.len() == 0 {
            let form: FormView = FormView::from(clap_app);
            fui = fui.action(clap_app.get_name(), "", form, |_| {});
        } else {
            for subcmd in clap_app.p.subcommands.iter() {
                let mut form: FormView = FormView::from(subcmd);
                // copy global fields to form
                form = copy_clap_fields_to_form(clap_app, form);
                fui = fui.action(
                    subcmd.get_name(),
                    subcmd.p.meta.about.unwrap_or(""),
                    form,
                    |_| {},
                );
            }
        }

        fui
    }
}

#[cfg(test)]
mod basic {
    use super::*;

    #[test]
    fn app_meta_data_test() {
        let app = clap::App::new("virtua_fighter")
            .about("Does awesome things")
            .author("Akria Yuki")
            .version("1.0");

        let fui = Fui::from(&app);

        assert_eq!(app.get_name(), fui.get_name());
        assert_eq!(app.p.meta.about, Some(fui.get_about()));
        assert_eq!(app.p.meta.author, Some(fui.get_author()));
        assert_eq!(app.p.meta.version, Some(fui.get_version()));
    }

    #[test]
    fn dump_as_cli_works_when_data_empty() {
        let app = clap::App::new("virtua_fighter");
        let fui = Fui::from(&app);

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter"]);
    }

    #[test]
    fn dump_as_cli_works_when_action_set() {
        let app = clap::App::new("virtua_fighter").subcommand(clap::SubCommand::with_name("first"));
        let mut fui = Fui::from(&app);
        fui.set_action("first");

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter", "first"]);
    }

    #[test]
    fn dump_as_cli_skips_default_subcommand() {
        let app_name = "virtua_fighter";
        let app = clap::App::new(app_name);
        let mut fui = Fui::from(&app);
        fui.set_action(app_name);

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter"]);
    }
}

#[cfg(test)]
mod switches {
    use Action;
    use Fui;

    #[test]
    fn dump_as_cli_works_when_checkbox_false_in_form() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("some-switch")
                .long("long")
                .help("arg-help"),
        );
        let mut fui = Fui::from(&app);
        fui.set_form_data(serde_json::from_str(r#"{ "long": false }"#).unwrap());

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter"]);
    }

    #[test]
    fn dump_as_cli_works_when_checkbox_true_in_form() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("some-switch")
                .long("long")
                .help("arg-help"),
        );
        let mut fui = Fui::from(&app);
        fui.set_form_data(serde_json::from_str(r#"{ "long": true }"#).unwrap());

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter", "--long"]);
    }

    #[test]
    fn basic_switch_is_converted_to_checkbox_test() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("some-switch")
                .long("arg_long")
                .help("arg_help"),
        );
        let fui = Fui::from(&app);

        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");

        let field = &action.form.as_ref().unwrap().get_fields()[0];

        assert_eq!(field.get_label(), "arg_long");
        assert_eq!(field.get_help(), "arg_help");
        //TODO: assert checkbox if possible
    }

    #[test]
    fn switch_multi_is_converted_to_text() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("some-switch")
                .long("arg_long")
                .help("arg_help")
                .multiple(true),
        );
        let fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");

        let field = &action.form.as_ref().unwrap().get_fields()[0];

        assert_eq!(field.get_label(), "arg_long");
        assert_eq!(field.get_help(), "arg_help");
        //TODO: assert text if possible
    }
}

#[cfg(test)]
mod option_args {
    use super::*;
    use Action;

    #[test]
    fn dump_as_cli_works_for_single_arg() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("arg-name")
                .takes_value(true)
                .long("long")
                .help("help"),
        );
        let mut fui = Fui::from(&app);
        fui.set_form_data(serde_json::from_str(r#"{ "long": "some value" }"#).unwrap());

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter", "--long", "some value"]);
    }

    #[test]
    fn field_respects_attribute_required_for_single_arg() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("some-option")
                .takes_value(true)
                .long("arg-long")
                .help("help")
                .required(true),
        );
        let fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");

        let field = &action.form.as_ref().unwrap().get_fields()[0];

        assert_eq!(field.is_required(), true);
    }

    #[test]
    fn field_respects_attribute_required_for_multi_args() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("some-option")
                .takes_value(true)
                .long("arg-long")
                .help("help")
                .required(true)
                .multiple(true),
        );
        let fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");

        let field = &action.form.as_ref().unwrap().get_fields()[0];

        assert_eq!(field.is_required(), true);
    }

    #[test]
    fn field_uses_default_value_if_present() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("option")
                .takes_value(true)
                .long("long")
                .help("help")
                .default_value("default"),
        );
        let fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");

        let initial = action.form.as_ref().unwrap().get_field_value("long");
        assert_eq!(initial, Some("default".to_string()));
    }

    #[test]
    fn field_uses_default_value_if_present_and_multiple() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("option")
                .takes_value(true)
                .long("long")
                .help("help")
                .default_value("default")
                .multiple(true),
        );
        let fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");

        let initial = action.form.as_ref().unwrap().get_field_value("long");
        assert_eq!(initial, Some("default".to_string()));
    }
}

#[cfg(test)]
mod positional_args {
    use super::*;
    use Action;

    #[test]
    fn name_is_numeric() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("some-switch")
                .index(1)
                .help("arg_help"),
        );
        let fui = Fui::from(&app);

        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");
        let field = &action.form.as_ref().unwrap().get_fields()[0];

        assert_eq!(field.get_label(), "1");
        assert_eq!(field.get_help(), "arg_help");
        //TODO: assert autocomplete if possible
    }

    #[test]
    fn dump_as_cli_works_for_single_arg() {
        let app = clap::App::new("virtua_fighter")
            .arg(clap::Arg::with_name("arg-name").help("help").index(1));
        let mut fui = Fui::from(&app);
        fui.set_form_data(serde_json::from_str(r#"{ "0": "some value" }"#).unwrap());

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter", "some value"]);
    }

    #[test]
    fn field_respects_attribute_required_for_single_arg() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("arg-name")
                .help("help")
                .index(1)
                .required(true),
        );
        let fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");

        let field = &action.form.as_ref().unwrap().get_fields()[0];

        assert_eq!(field.is_required(), true);
    }

    #[test]
    fn field_respects_attribute_required_for_multi_args() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("arg-name")
                .help("help")
                .index(1)
                .required(true)
                .multiple(true),
        );
        let fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");

        let field = &action.form.as_ref().unwrap().get_fields()[0];

        assert_eq!(field.is_required(), true);
    }

    #[test]
    fn field_uses_default_value_if_present() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("option")
                .help("help")
                .index(0)
                .default_value("default"),
        );
        let fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");

        let initial = action.form.as_ref().unwrap().get_field_value("0");
        assert_eq!(initial, Some("default".to_string()));
    }

    #[test]
    fn field_uses_default_value_if_present_and_multiple() {
        let app = clap::App::new("virtua_fighter").arg(
            clap::Arg::with_name("option")
                .help("help")
                .index(0)
                .default_value("default")
                .multiple(true),
        );
        let fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");

        let initial = action.form.as_ref().unwrap().get_field_value("0");
        assert_eq!(initial, Some("default".to_string()));
    }

}

#[cfg(test)]
mod subcommands {
    use super::*;
    use clap;
    use Action;

    #[test]
    fn dump_as_cli_works_when_checkbox_in_subcommand() {
        let app = clap::App::new("virtua_fighter").subcommand(
            clap::SubCommand::with_name("first").about("about").arg(
                clap::Arg::with_name("subcmd-name")
                    .long("subcmd-long")
                    .help("subcmd-help"),
            ),
        );
        let mut fui = Fui::from(&app);
        fui.set_action("first");
        fui.set_form_data(serde_json::from_str(r#"{ "subcmd-long": true }"#).unwrap());

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter", "first", "--subcmd-long"]);
    }

    #[test]
    fn zero_subcmds_creates_default_command_test() {
        let app = clap::App::new("virtua_fighter");
        let fui = Fui::from(&app);

        let found = fui.actions().iter().map(|a| a.name).collect::<Vec<&str>>();

        assert_eq!(found, vec!["virtua_fighter"]);
    }

    #[test]
    fn n_subcmds_creates_n_command_test() {
        let app = clap::App::new("virtua_fighter")
            .subcommand(clap::SubCommand::with_name("first"))
            .subcommand(clap::SubCommand::with_name("second"));
        let fui = Fui::from(&app);

        let found = fui.actions().iter().map(|a| a.name).collect::<Vec<&str>>();

        assert_eq!(found, vec!["first", "second"]);
    }

    #[test]
    fn global_flag_is_propagated_to_subcommand() {
        let app = clap::App::new("virtua_fighter")
            .arg(
                clap::Arg::with_name("global-flag-name")
                    .long("global-flag-long")
                    .help("global-flag-help")
                    .global(true),
            )
            .subcommand(clap::SubCommand::with_name("first"));
        let fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("first")
            .expect("expected action second exists");

        let field = &action.form.as_ref().unwrap().get_fields()[0];

        assert_eq!(field.get_label(), "global-flag-long");
        assert_eq!(field.get_help(), "global-flag-help");
    }

    #[test]
    fn global_positional_is_propagated_to_subcommand() {
        let app = clap::App::new("virtua_fighter")
            .arg(
                clap::Arg::with_name("global-positional-name")
                    .index(1)
                    .long("global-positional-long")
                    .help("global-positional-help")
                    .global(true),
            )
            .subcommand(clap::SubCommand::with_name("first"));
        let fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("first")
            .expect("expected action second exists");

        let field = &action.form.as_ref().unwrap().get_fields()[0];

        assert_eq!(field.get_label(), "1");
        assert_eq!(field.get_help(), "global-positional-help");
    }

    #[test]
    fn global_option_is_propagated_to_subcommand() {
        let app = clap::App::new("virtua_fighter")
            .arg(
                clap::Arg::with_name("global-option-name")
                    .long("global-option-long")
                    .help("global-option-help")
                    .takes_value(true)
                    .global(true),
            )
            .subcommand(clap::SubCommand::with_name("first"));
        let fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("first")
            .expect("expected action second exists");

        let field = &action.form.as_ref().unwrap().get_fields()[0];

        assert_eq!(field.get_label(), "global-option-long");
        assert_eq!(field.get_help(), "global-option-help");
    }
}

#[cfg(test)]
mod split_values {
    use super::*;

    #[test]
    fn split_values_works() {
        assert_eq!(split_values("abc", ' '), vec!["abc"]);
        assert_eq!(split_values("a b", ' '), vec!["a", "b"]);
        assert_eq!(split_values("\"a b\"", ' '), vec!["a b"]);
        assert_eq!(split_values("\"a b\" \"c d\"", ' '), vec!["a b", "c d"]);
        // TODO:: fix this case
        //assert_eq!(
        //    split_values("\"a b\" c d", ' '), vec!["a b", "c", "d"]
        //);
    }
}
