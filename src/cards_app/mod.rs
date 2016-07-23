extern crate cards;

use assets;
use cards::card::Card;
use cards::deck::Deck;
use std::sync::Arc;
use std::collections::HashMap;

type CardTextures = HashMap<Card, Arc<G2dTexture<'static>>>;

pub struct CardsApp {
    card: Option<Card>,
    card_textures: CardTextures,
    back_texture: Arc<G2dTexture<'static>>,
}

use piston_window::{G2dTexture, PistonWindow};
fn generate_textures(window: &mut PistonWindow) -> CardTextures {
    let cards = Deck::new_unshuffled().draw_n(52).ok().unwrap();
    let mut textures = HashMap::with_capacity(52);
    for c in cards {
        textures.insert(c.clone(), assets::card(window, c));
    }
    textures
}

impl CardsApp {
    pub fn new(window: &mut PistonWindow) -> CardsApp {
        CardsApp {
            card: None,
            card_textures: generate_textures(window),
            back_texture: assets::hidden_card(window),
        }
    }
    pub fn add_card(&mut self, card: Card) {
        self.card = Some(card);
    }
    pub fn last_card(&self) -> Option<Card> {
        self.card
    }
    pub fn card_texture(&self) -> Arc<G2dTexture<'static>> {
        match self.card {
            Some(card) => {
                println!("{}", card);
                self.card_textures[&card].clone()
            }
            None => {
                println!("hidden");
                self.back_texture.clone()
            }
        }
    }
}
