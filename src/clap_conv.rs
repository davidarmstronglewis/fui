//!
//! This is an EXPERIMENTAL feature on very early stage.
//!

use clap;
use clap::ArgSettings;
use feeders::DirItems;
use fields::{Autocomplete, Checkbox, Multiselect, Text};
use form::FormView;
use validators::Required;
use Fui;

fn show_warn(msg: &'static str) {
    // TODO: find a better way for warning users
    // crate log requires to use env var to make messages visible
    // so we need something better
    panic!(msg);
}

impl<'a> From<&'a clap::App<'_, '_>> for FormView {
    fn from(clap_app: &'a clap::App) -> Self {
        let mut form = FormView::new();
        // TODO: flag & option loops are mostly copy & paste so make it DRY
        for option in clap_app.p.opts.iter() {
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
                let mut field = Multiselect::new(long, DirItems::new()).help(help);
                if option.b.settings.is_set(ArgSettings::Required) {
                    field = field.validator(Required);
                }
                form = form.field(field)
            } else {
                let mut field = Autocomplete::new(long, DirItems::new()).help(help);
                if option.b.settings.is_set(ArgSettings::Required) {
                    field = field.validator(Required);
                }
                form = form.field(field)
            }
        }
        for flag in clap_app.p.flags.iter() {
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
                form = form.field(Text::new(long).help(help));
            } else {
                form = form.field(Checkbox::new(long).help(help));
            }
        }
        form
    }
}

impl<'a> From<&'a clap::App<'_, '_>> for Fui<'a, 'a> {
    fn from(clap_app: &'a clap::App) -> Self {
        let mut fui = Fui::new(clap_app.get_name())
            .about(clap_app.p.meta.about.unwrap_or(""))
            .author(clap_app.p.meta.author.unwrap_or(""))
            .version(clap_app.p.meta.version.unwrap_or(""));

        //println!("{:?}", clap_app.p.flags);

        if clap_app.p.subcommands.len() == 0 {
            let form: FormView = FormView::from(clap_app);
            fui = fui.action(clap_app.get_name(), "", form, |_| {});
        } else {
            for subcmd in clap_app.p.subcommands.iter() {
                let form: FormView = FormView::from(subcmd);
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
mod tests {
    use super::*;
    use clap::{App, Arg, SubCommand};
    use Action;

    #[test]
    fn app_meta_data_test() {
        let app = App::new("virtua_fighter")
            .about("Does awesome things")
            .author("Akria Yuki")
            .version("1.0");

        let fui: Fui = Fui::from(&app);

        assert_eq!(app.get_name(), fui.get_name());
        assert_eq!(app.p.meta.about, Some(fui.get_about()));
        assert_eq!(app.p.meta.author, Some(fui.get_author()));
        assert_eq!(app.p.meta.version, Some(fui.get_version()));
    }

    #[test]
    fn dump_as_cli_works_when_data_empty() {
        let app = App::new("virtua_fighter");
        let fui = Fui::from(&app);

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter"]);
    }

    #[test]
    fn dump_as_cli_works_when_action_set() {
        let app = App::new("virtua_fighter").subcommand(SubCommand::with_name("first"));
        let mut fui = Fui::from(&app);
        fui.set_action("first");

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter", "first"]);
    }

    #[test]
    fn dump_as_cli_skips_default_subcommand() {
        let app_name = "virtua_fighter";
        let app = App::new(app_name);
        let mut fui = Fui::from(&app);
        fui.set_action(app_name);

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter"]);
    }

    #[test]
    fn dump_as_cli_works_when_checkbox_false_in_form() {
        let app = App::new("virtua_fighter")
            .arg(Arg::with_name("some-switch").long("long").help("arg-help"));
        let mut fui = Fui::from(&app);
        fui.set_form_data(serde_json::from_str(r#"{ "long": false }"#).unwrap());

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter"]);
    }

    #[test]
    fn dump_as_cli_works_when_checkbox_true_in_form() {
        let app = App::new("virtua_fighter")
            .arg(Arg::with_name("some-switch").long("long").help("arg-help"));
        let mut fui = Fui::from(&app);
        fui.set_form_data(serde_json::from_str(r#"{ "long": true }"#).unwrap());

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter", "--long"]);
    }

    #[test]
    fn dump_as_cli_works_when_checkbox_in_subcommand() {
        let app = App::new("virtua_fighter").subcommand(
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
        let app = App::new("virtua_fighter");
        let fui: Fui = Fui::from(&app);

        let found = fui.actions().iter().map(|a| a.name).collect::<Vec<&str>>();

        assert_eq!(found, vec!["virtua_fighter"]);
    }

    #[test]
    fn n_subcmds_creates_n_command_test() {
        let app = App::new("virtua_fighter")
            .subcommand(SubCommand::with_name("first"))
            .subcommand(SubCommand::with_name("second"));
        let fui: Fui = Fui::from(&app);

        let found = fui.actions().iter().map(|a| a.name).collect::<Vec<&str>>();

        assert_eq!(found, vec!["first", "second"]);
    }

    #[test]
    fn basic_switch_is_converted_to_checkbox_test() {
        let app = App::new("virtua_fighter").arg(
            Arg::with_name("some-switch")
                .long("arg_long")
                .help("arg_help"),
        );
        let fui: Fui = Fui::from(&app);

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
        let app = App::new("virtua_fighter").arg(
            Arg::with_name("some-switch")
                .long("arg_long")
                .help("arg_help")
                .multiple(true),
        );
        let fui: Fui = Fui::from(&app);
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
    use clap::{App, Arg};
    use Action;

    #[test]
    fn dump_as_cli_works_for_single_option() {
        let app = App::new("virtua_fighter").arg(
            Arg::with_name("arg-name")
                .takes_value(true)
                .long("long")
                .help("help"),
        );
        let mut fui = Fui::from(&app);
        fui.set_form_data(serde_json::from_str(r#"{ "long": "some-value" }"#).unwrap());

        let dumped = fui.dump_as_cli();

        assert_eq!(dumped, vec!["virtua_fighter", "--long", "\"some-value\""]);
    }

    #[test]
    fn field_respects_attribute_required_for_single_option() {
        let app = App::new("virtua_fighter").arg(
            Arg::with_name("some-option")
                .takes_value(true)
                .long("arg-long")
                .help("help")
                .required(true),
        );
        let fui: Fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");

        let field = &action.form.as_ref().unwrap().get_fields()[0];

        assert_eq!(field.is_required(), true);
    }

    #[test]
    fn field_respects_attribute_required_for_multi_option() {
        let app = App::new("virtua_fighter").arg(
            Arg::with_name("some-option")
                .takes_value(true)
                .long("arg-long")
                .help("help")
                .required(true)
                .multiple(true),
        );
        let fui: Fui = Fui::from(&app);
        let action: &Action = fui
            .action_by_name("virtua_fighter")
            .expect("expected default action");

        let field = &action.form.as_ref().unwrap().get_fields()[0];

        assert_eq!(field.is_required(), true);
    }
}
