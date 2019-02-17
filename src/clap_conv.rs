//!
//! This is an EXPERIMENTAL feature on very early stage.
//!

use crate::Fui;


impl<'attrs, 'action> From<&'attrs clap::App<'attrs, 'action>> for Fui<'attrs, 'action> {
    fn from(clap_app: &'attrs clap::App) -> Self {
        Fui::new(clap_app.get_name())
            .about(clap_app.get_about().unwrap_or(""))
            .author(clap_app.get_author().unwrap_or(""))
            .version(clap_app.get_version().unwrap_or(""))
    }
}

#[cfg(test)]
mod tests {
    use clap::App;
    use super::*;

    #[test]
    fn app_meta_data_test() {
        let app = App::new("MyApp")
            .about("Does awesome things")
            .author("Akria Yuki")
            .version("1.0");
        let fui: Fui = Fui::from(&app);

        assert_eq!(app.get_name(), fui.get_name());
        assert_eq!(app.get_about(), Some(fui.get_about()));
        assert_eq!(app.get_author(), Some(fui.get_author()));
        assert_eq!(app.get_version(), Some(fui.get_version()));
    }
}
