// Usage example of view Multiselect
extern crate cursive;
extern crate fui;

use std::rc::Rc;

use cursive::Cursive;
use cursive::views::LinearLayout;

use fui::feeders::DirItems;
use fui::views::Multiselect;

fn handler(_c: &mut Cursive, kind: &str, value: Rc<String>) {
    let text = format!("{:?}: {:?}", kind, value);
    // uncomment it to see data without redirecting errors stream, like:
    // cargo run --example view_multiselect 2>errors.log
    //c.add_layer(Dialog::info(text.clone()));
    eprintln!("{:?}", text);
}

fn main() {
    let mut c = Cursive::default();

    let widget = LinearLayout::vertical()
        .child(
            Multiselect::new(DirItems::new())
                .on_select(|c, text| handler(c, "on_select", text))
                .on_deselect(|c, text| handler(c, "on_deselect", text)),
        )
        .child(
            Multiselect::new(DirItems::new())
                // allows to select items out of completition
                .select_anything()
                .on_select(|c, text| handler(c, "on_select", text))
                .on_deselect(|c, text| handler(c, "on_deselect", text)),
        )
        .child(
            Multiselect::new(DirItems::new())
                // allows to select single item many times
                .redundant_selection()
                .on_select(|c, text| handler(c, "on_select", text))
                .on_deselect(|c, text| handler(c, "on_deselect", text)),
        );

    c.add_layer(widget);

    c.run();
}
