extern crate cards_ui;

use cards_ui::{CardsApp, CardsUi};

fn main() {
    CardsUi::new("Hidden card").run(CardsApp::new());
}
