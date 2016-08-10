#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

#[macro_use]
extern crate conrod;
extern crate piston_window;
pub extern crate cards;

mod backend;
mod assets;
mod window;
mod cards_app;

pub use cards::card::{Card, Value, Suit};
use cards_app::CardsApp;
use piston_window::PistonWindow;

pub struct CardsUi {
    app: CardsApp,
    window: PistonWindow,
}

impl CardsUi {
    pub fn new(title: &'static str) -> CardsUi {
        CardsUi {
            app: CardsApp::new(),
            window: window::setup(title),
        }
    }
    pub fn add_card(&mut self, card: Card) -> &mut CardsUi {
        self.app.add_card(card);
        self
    }
    pub fn add_deck(&mut self) -> &mut CardsUi {
        self.app.add_deck();
        self
    }
    pub fn flip(&mut self) -> &mut CardsUi {
        self.app.flip();
        self
    }
    pub fn run(&mut self) {
        use piston_window::{EventLoop, UpdateEvent};

        let mut ui = assets::conrod_ui();
        let mut text_texture_cache = assets::text_texture_cache(&mut self.window);

        let mut image_map = image_map! {
            (CARD, self.app.texture(&mut self.window)),
        };

        self.window.set_ups(60);
        // Poll events from the window.
        while let Some(event) = self.window.next() {
            // Convert the piston event to a conrod event.
            if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(),
                                                                           &self.window) {
                ui.handle_event(e);
            }

            self.window.draw_2d(&event, |c, g| {
                if let Some(primitives) = ui.draw_if_changed() {
                    fn texture_from_image<T>(img: &T) -> &T {
                        img
                    };
                    use conrod::backend::piston_window::draw;
                    draw(c,
                         g,
                         primitives,
                         &mut text_texture_cache,
						 &image_map,
                         texture_from_image);
                }
            });
            event.update(|_| ui.set_widgets(|mut ui| self.set_widgets(&mut ui, &mut image_map)));
        }
    }
    fn set_widgets(&mut self,
                   ui: &mut backend::UiCell,
                   image_map: &mut conrod::image::Map<piston_window::G2dTexture<'static>>) {
        use conrod::{Borderable, Colorable, Labelable, Positionable, Sizeable, Texturable, Widget, color};
        use conrod::widget::{Canvas, Image, Index, Button};
		Canvas::new().color(color::LIGHT_BLUE).set(CANVAS, ui);
        use piston_window::ImageSize;
        let (w, h) = image_map.get(CARD).unwrap().get_size();
        Image::new()
            .w_h(w as f64, h as f64)
            .set(CARD, ui);
        Button::new()
            .rgb(0.4, 0.75, 0.6)
            .label("Pop")
            .mid_left_of(CANVAS)
            .react(|| {
                self.app.pop();
                image_map.insert(CARD, self.app.texture(&mut self.window));
            })
            .set(POP_BUTTON, ui);
        Button::new()
		    .color(color::LIGHT_BLUE)
		    .border_color(color::LIGHT_BLUE)
            .middle_of(CANVAS)
			.texture(Index::from(CARD))
            .react(|| {
                self.app.flip();
                image_map.insert(CARD, self.app.texture(&mut self.window));
            })
            .set(FLIP_BUTTON, ui);
    }
}

widget_ids! {
    CANVAS,
    CARD,
    POP_BUTTON,
    FLIP_BUTTON,
}
