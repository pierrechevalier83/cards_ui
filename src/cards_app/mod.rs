extern crate cards;

use cards::card::Card;

#[derive(Clone)]
pub struct CardsApp {
    card: Option<Card>,
}

impl CardsApp {
    pub fn new() -> CardsApp {
        CardsApp { card: None }
    }
    pub fn add_card(mut self, card: Card) -> CardsApp {
        self.card = Some(card);
        self
    }
    pub fn last_card(self) -> Option<Card> {
        self.card
    }
}
