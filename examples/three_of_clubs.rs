extern crate cards_ui;

use cards_ui::{Card, CardsApp, CardsUi, Value, Suit, window};

fn main() {
    let mut window = window::setup("Three of clubs");

    let mut app = CardsApp::new();
    let three_of_club = Card::new(Value::Three, Suit::Clubs);
    app.add_card(three_of_club);

    CardsUi::new().run(&mut window, &mut app);
}
