extern crate cards;
extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use std::path::PathBuf;

use piston_window::{Flip, G2dTexture, PistonWindow, Texture, TextureSettings};
pub fn hidden_card(window: &mut PistonWindow) -> G2dTexture<'static> {
    let path = assets_folder().join("images/card-deck/Back Covers/Emerald.png");
    let factory = &mut window.factory;
    let settings = TextureSettings::new();
    Texture::from_path(factory, &path, Flip::None, &settings).unwrap()
}

pub fn rust_logo(window: &mut PistonWindow) -> G2dTexture<'static> {
    let path = assets_folder().join("images/rust.png");
    let factory = &mut window.factory;
    let settings = TextureSettings::new();
    Texture::from_path(factory, &path, Flip::None, &settings).unwrap()
}

use cards::card::Card;
pub fn card(window: &mut PistonWindow, card: Card) -> G2dTexture<'static> {
    let path = asset_path(card);
    let factory = &mut window.factory;
    let settings = TextureSettings::new();
    Texture::from_path(factory, &path, Flip::None, &settings).unwrap()
}

use super::backend;
pub fn conrod_ui() -> backend::Ui {
    backend::Ui::new(conrod::Theme::default())
}

pub fn text_texture_cache(window: &mut PistonWindow) -> conrod::backend::piston_window::GlyphCache {
    // let font_path = assets_folder().join("fonts/NotoSans/NotoSans-Regular.ttf");
    conrod::backend::piston_window::GlyphCache::new(window, 0, 0)
}

fn assets_folder() -> PathBuf {
    // Get the path to our `assets` directory (where the fonts and images are).
    find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap()
}

fn asset_path(card: Card) -> PathBuf {
    let mut path = assets_folder();
    path.push("images");
    path.push("card-deck");
    path.push(suit_asset_folder_name(card.suit));
    path.push(value_asset_folder_name(card.value));
    path.set_extension("png");
    path
}

use cards::card::Suit;
fn suit_asset_folder_name(suit: Suit) -> &'static str {
    match suit {
        Suit::Spades => "Spades",
        Suit::Hearts => "Hearts",
        Suit::Diamonds => "Diamonds",
        Suit::Clubs => "Clubs",
    }
}

use cards::card::Value;
fn value_asset_folder_name(value: Value) -> &'static str {
    match value {
        Value::Two => "2",
        Value::Three => "3",
        Value::Four => "4",
        Value::Five => "5",
        Value::Six => "6",
        Value::Seven => "7",
        Value::Eight => "8",
        Value::Nine => "9",
        Value::Ten => "10",
        Value::Jack => "J",
        Value::Queen => "Q",
        Value::King => "K",
        Value::Ace => "A",
    }
}
