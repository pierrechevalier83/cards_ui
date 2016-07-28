extern crate cards_ui;

use cards_ui::{CardsUi, window};

fn main() {
    let mut window = window::setup("No card");
    CardsUi::new().run(&mut window);
}
