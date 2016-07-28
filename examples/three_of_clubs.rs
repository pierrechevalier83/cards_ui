extern crate cards_ui;

use cards_ui::{Card, CardsUi, Value, Suit};

fn main() {
    let three_of_club = Card::new(Value::Three, Suit::Clubs);
    CardsUi::new("Three of clubs").add_card(three_of_club).flip().run();
}
