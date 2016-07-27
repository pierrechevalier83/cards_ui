extern crate cards_ui;

use cards_ui::{CardsApp, CardsUi, window};

fn main() {
    let mut window = window::setup("No card");
    let mut app = CardsApp::new();
    CardsUi::new().run(&mut window, &mut app);
}
