use fui::Fui;
use fui::fields::Text;
use fui::form::FormView;

pub fn build_app() -> Fui<'static, 'static> {
    Fui::new()
        .action(
            "action1",
            "help for action1",
            FormView::new().field(Text::new("action1-data").help("help for action1 data")),
            |v| {
                println!("user input (from closure) {:?}", v);
            },
        )
        .name(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
}
