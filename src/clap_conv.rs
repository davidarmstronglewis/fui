//!
//! This is an EXPERIMENTAL feature on very early stage.
//!

use Fui;
use clap;
use fields::Checkbox;
use form::FormView;

impl<'a> From<&'a clap::App<'_, '_>> for Fui<'a, 'a> {
    fn from(clap_app: &'a clap::App) -> Self {
        let mut fui = Fui::new(clap_app.get_name())
            .about(clap_app.get_about().unwrap_or(""))
            .author(clap_app.get_author().unwrap_or(""))
            .version(clap_app.get_version().unwrap_or(""));

        if clap_app.p.subcommands.len() == 0 {

            let mut form = FormView::new();
            for flag in clap_app.p.flags.iter() {
                let long = flag.s.long.unwrap();
                let help = flag.b.help.unwrap();
                form = form.field(Checkbox::new(long).help(help));
            }

            fui = fui.action(
                "default",
                "Auto generated subcommand for compatibility",
                form,
                |_| {},
            );

        } else {
            for subcmd in clap_app.p.subcommands.iter() {
                fui = fui.action(
                    subcmd.get_name(),
                    subcmd.p.meta.about.unwrap_or(""),
                    //TODO:
                    FormView::new(),
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
    use Action;
    use clap::{App, Arg, SubCommand};

    #[test]
    fn app_meta_data_test() {
        let app = App::new("virtua_fighter")
            .about("Does awesome things")
            .author("Akria Yuki")
            .version("1.0");
        let fui: Fui = Fui::from(&app);

        assert_eq!(app.get_name(), fui.get_name());
        assert_eq!(app.get_about(), Some(fui.get_about()));
        assert_eq!(app.get_author(), Some(fui.get_author()));
        assert_eq!(app.get_version(), Some(fui.get_version()));
    }

    #[test]
    fn zero_subcmds_creates_default_command_test() {
        let app = App::new("virtua_fighter");
        let fui: Fui = Fui::from(&app);
        let found = fui.actions().iter().map(|a| a.name).collect::<Vec<&str>>();
        assert_eq!(found, vec!["default"]);
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
                .help("arg_help")
        );
        let fui: Fui = Fui::from(&app);

        let action: &Action = fui.action_by_name("default")
            .expect("expected default action");
        let field = &action.form.as_ref().unwrap().get_fields()[0];
        assert_eq!(field.get_label(), "arg_long");
        assert_eq!(field.get_help(), "arg_help");
        //TODO::: assert checkbox if possible
    }
    //TODO:::
    //.multiple(true)// use
    //.requires("config") //warning
    //.conflicts_with("output")// warning
    // required
        //.help("turns up the awesome")
        //.long("awesome")

}
