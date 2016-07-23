extern crate cards;

use assets;
use cards::card::Card;
use std::sync::Arc;

#[derive(Clone)]
pub struct CardsApp {
    card: Option<Card>,
}

use piston_window::{G2dTexture, PistonWindow};
impl CardsApp {
    pub fn new() -> CardsApp {
        CardsApp { card: None }
    }
    pub fn add_card(&mut self, card: Card) {
        self.card = Some(card);
    }
    pub fn last_card(&self) -> Option<Card> {
        self.card
    }
    pub fn card_texture(&self, window: &mut PistonWindow) -> Arc<G2dTexture<'static>> {
        match self.card {
            Some(card) => assets::card(window, card),
            None => assets::hidden_card(window),
        }
    }
}
