extern crate cards_ui;

use cards_ui::{Card, CardsUi, Value, Suit};

fn main() {
    let three_of_club = Card::new(Value::Three, Suit::Clubs);
    CardsUi::new("Hidden card").add_card(three_of_club).run();
}
