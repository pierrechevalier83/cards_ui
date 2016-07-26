extern crate cards;

use assets;
use cards::card::Card;
// use cards::deck::Deck;
use std::collections::HashMap;

use piston_window::{G2dTexture, PistonWindow};
type CardTextures = HashMap<Card, G2dTexture<'static>>;

pub struct CardsApp {
    card: Option<Card>,
}

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
    pub fn card_texture(&self, window: &mut PistonWindow) -> G2dTexture<'static> {
        match self.card {
            Some(card) => {
                println!("{}", card);
                assets::card(window, card)
            }
            None => {
                println!("hidden");
                assets::hidden_card(window)
            }
        }
    }
}
