extern crate cards_ui;

use cards_ui::{Card, CardsApp, CardsUi, Value, Suit};

fn main() {
    let three_of_club = Card::new(Value::Three, Suit::Clubs);
    let mut app = CardsApp::new();
    app.add_card(three_of_club);
    CardsUi::new("Three of clubs").run(&mut app);
}
