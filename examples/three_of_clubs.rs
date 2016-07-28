extern crate cards_ui;

use cards_ui::{Card, CardsUi, Value, Suit, window};

fn main() {
    let mut window = window::setup("Three of clubs");
    let three_of_club = Card::new(Value::Three, Suit::Clubs);
    CardsUi::new().add_card(three_of_club).flip().run(&mut window);
}
