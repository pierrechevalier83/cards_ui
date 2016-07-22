extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use std::sync::Arc;
use std::path::PathBuf;

use piston_window::{Flip, G2dTexture, PistonWindow, Texture, TextureSettings};
pub fn rust_logo(window: &mut PistonWindow) -> Arc<G2dTexture<'static>> {
    let path = assets_folder().join("images/rust.png");
    let factory = &mut window.factory;
    let settings = TextureSettings::new();
    Arc::new(Texture::from_path(factory, &path, Flip::None, &settings).unwrap())
}

use super::backend;
pub fn conrod_ui(window: &mut PistonWindow) -> backend::Ui {
    let font_path = assets_folder().join("fonts/NotoSans/NotoSans-Regular.ttf");
    let theme = conrod::Theme::default();
    let glyph_cache = piston_window::Glyphs::new(&font_path, window.factory.clone()).unwrap();
    backend::Ui::new(glyph_cache, theme)
}

fn assets_folder() -> PathBuf {
    // Get the path to our `assets` directory (where the fonts and images are).
    find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap()
}
