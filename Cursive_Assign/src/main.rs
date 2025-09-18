use cursive::views::{Dialog, TextView};
use cursive::{Cursive, CursiveExt};

fn main() {
    let mut siv = Cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("Hello, Josh we are Cursive right now! 🚀"))
            .title("Cursive + Cross term")
            .button("Quit", |s| s.quit()),
    );

    siv.run();
}
