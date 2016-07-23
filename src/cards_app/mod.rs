extern crate cards;

use cards::card::{Card, Value, Suit};

pub enum Visibility {
    visible,
    hidden,
}

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
    pub fn visibility(self) -> Visibility {
        match self.card {
            Some(card) => Visibility::visible,
            None => Visibility::hidden,
        }
    }
    pub fn last_card(self) -> Option<Card> {
        self.card
    }
}
