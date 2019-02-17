// Usage example of feeders
extern crate cursive;
extern crate fui;

use std::rc::Rc;

use cursive::traits::Boxable;
use cursive::views::{Dialog, DummyView, LinearLayout};
use cursive::Cursive;

use fui::feeders::DirItems;
use fui::views::Autocomplete;

fn handler(c: &mut Cursive, submitted: Rc<String>) {
    let text = format!("submitted {:?}", submitted);
    c.add_layer(Dialog::info(text));
    //eprintln!("{:?}", text);
}

fn main() {
    let mut c = Cursive::default();

    let layout = LinearLayout::vertical()
        .child(
            Autocomplete::new(vec![
                "option1", "option2", "option3", "option4", "option5", "option6",
            ])
            .on_submit(handler),
        )
        .child(DummyView)
        .child(Autocomplete::new(DirItems::new()).on_submit(handler))
        .child(DummyView)
        // completes paths as absolute paths
        .child(Autocomplete::new(DirItems::new().use_full_paths()).on_submit(handler))
        .child(DummyView);

    c.add_layer(Dialog::around(layout).full_width());

    c.run();
}
