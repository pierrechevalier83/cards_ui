extern crate cards;

use assets;
use cards::card::Card;
// use cards::deck::Deck;

use piston_window::{G2dTexture, PistonWindow};

enum VisibleFace {
    Up,
    Down,
}

pub struct CardsApp {
    card: Option<Card>,
    visible: VisibleFace,
}

impl CardsApp {
    pub fn new() -> CardsApp {
        CardsApp {
            card: None,
            visible: VisibleFace::Down,
        }
    }
    pub fn add_card(&mut self, card: Card) {
        self.card = Some(card);
    }
    pub fn last_card(&self) -> Option<Card> {
        self.card
    }
    pub fn flip(&mut self) {
        self.visible = match self.visible {
            VisibleFace::Up => VisibleFace::Down,
            VisibleFace::Down => VisibleFace::Up,
        };
    }
    pub fn texture(&self, window: &mut PistonWindow) -> G2dTexture<'static> {
        match self.visible {
            VisibleFace::Up => self.card_face_up(window),
            VisibleFace::Down => self.card_face_down(window),
        }
    }
    fn card_face_up(&self, window: &mut PistonWindow) -> G2dTexture<'static> {
        match self.card {
            Some(card) => assets::card(window, card),
            None => assets::rust_logo(window),
        }
    }
    fn card_face_down(&self, window: &mut PistonWindow) -> G2dTexture<'static> {
        match self.card {
            Some(_) => assets::hidden_card(window),
            None => assets::rust_logo(window),
        }
    }
}
