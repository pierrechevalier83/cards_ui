extern crate cards;

use assets;
use cards::card::Card;
// use cards::deck::Deck;

use piston_window::{G2dTexture, PistonWindow};

#[derive(Clone)]
enum VisibleFace {
    Up,
    Down,
}

pub struct SingleCard {
    card: Card,
    visible: VisibleFace,
}

impl SingleCard {
    fn new(card: Card) -> SingleCard {
        SingleCard {
            card: card,
            visible: VisibleFace::Down,
        }
    }
    fn flip(&mut self) {
        self.visible = match self.visible {
            VisibleFace::Up => VisibleFace::Down,
            VisibleFace::Down => VisibleFace::Up,
        };
    }
}

enum StackLayout {
    Stacked,
    Expanded, // TODO (for solitary style games)
}

pub struct StackOfCards {
    stack: Vec<SingleCard>,
    layout: StackLayout, // TODO
}

impl StackOfCards {
    fn new() -> StackOfCards {
        StackOfCards {
            stack: Vec::new(),
            layout: StackLayout::Stacked,
        }
    }
    fn push(&mut self, card: Card) {
        self.stack.push(SingleCard::new(card));
    }
    fn last(&self) -> Option<Card> {
        match self.stack.last() {
            Some(last) => Some(last.card),
            None => None,
        }
    }
    fn visible(&self) -> Option<VisibleFace> {
        match self.stack.last() {
            Some(last) => Some(last.visible.clone()),
            None => None,
        }
    }
    fn flip(&mut self) {
        for card in &mut self.stack {
            card.flip();
        }
    }
}

pub struct CardsApp {
    stack: StackOfCards,
}

impl CardsApp {
    pub fn new() -> CardsApp {
        CardsApp { stack: StackOfCards::new() }
    }
    pub fn add_card(&mut self, card: Card) {
        self.stack.push(card);
    }
    pub fn last_card(&self) -> Option<Card> {
        self.stack.last()
    }
    pub fn flip(&mut self) {
        self.stack.flip();
    }
    pub fn texture(&self, window: &mut PistonWindow) -> G2dTexture<'static> {
        match self.stack.visible() {
            Some(v) => {
                match v {
                    VisibleFace::Up => assets::card(window, self.last_card().unwrap()),
                    VisibleFace::Down => assets::hidden_card(window),
                }
            }
            None => assets::rust_logo(window),
        }
    }
}
