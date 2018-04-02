// Usage example of view Autocomplete
extern crate cursive;
extern crate fui;

use std::rc::Rc;

use cursive::Cursive;
use cursive::traits::Boxable;
use cursive::views::{Dialog, DummyView, LinearLayout};

use fui::feeders::DirItems;
use fui::views::Autocomplete;

fn handler(c: &mut Cursive, submitted: Rc<String>) {
    let text = format!("submitted {:?}", submitted);
    c.add_layer(Dialog::info(text));
    //eprintln!("{:?}", text);
}

fn main() {
    let mut c = Cursive::default();

    let widget = LinearLayout::vertical()
        .child(Autocomplete::new(DirItems::new()).on_submit(handler))
        .child(DummyView)
        .child(
            Autocomplete::new(DirItems::new())
            // allow submitting values outside completition
            .submit_anything()
            .on_submit(handler),
        )
        .full_width();

    c.add_layer(widget);

    c.run();
}
