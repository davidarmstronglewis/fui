//!
//! This is an EXPERIMENTAL feature on very early stage.
//!

use crate::Fui;
use clap;
use form::FormView;

impl<'a> From<&'a clap::App<'_, '_>> for Fui<'a, 'a> {
    fn from(clap_app: &'a clap::App) -> Self {
        let mut fui = Fui::new(clap_app.get_name())
            .about(clap_app.get_about().unwrap_or(""))
            .author(clap_app.get_author().unwrap_or(""))
            .version(clap_app.get_version().unwrap_or(""));

        if clap_app.p.subcommands.len() == 0 {
            fui = fui.action(
                "default",
                "Auto generated subcommand for compatibility",
                //TODO:
                FormView::new(),
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
    use clap::{App, SubCommand};

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
}
