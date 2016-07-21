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
// TODO: private once all assets have moved to this mod
pub fn assets_folder() -> PathBuf {
    // Get the path to our `assets` directory (where the fonts and images are).
    find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap()
}
