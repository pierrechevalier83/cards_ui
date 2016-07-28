extern crate cards_ui;

use cards_ui::CardsUi;

fn main() {
    CardsUi::new("A 52 cards deck").add_deck().run();
}
