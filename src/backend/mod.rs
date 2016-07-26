extern crate conrod;
extern crate piston_window;

// Conrod is backend agnostic. Here, we define the `piston_window` backend to use for our `Ui`.
pub type Backend = (piston_window::G2dTexture<'static>, piston_window::Glyphs);
pub type Ui = conrod::Ui;
pub type UiCell<'a> = conrod::UiCell<'a>;
