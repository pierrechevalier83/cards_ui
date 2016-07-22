extern crate cards_ui;

use cards_ui::*;

fn main() {
    let ui = CardsUi::new("Three of clubs").add_card(Card::new(Value::Three, Suit::Clubs));
    ui.run();
}
