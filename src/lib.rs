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

pub use cards::card::{Card, Value, Suit};

pub struct CardsUi {
    // TODO: layout
    title: &'static str,
    card: Option<Card>,
}

impl CardsUi {
    pub fn new(t: &'static str) -> CardsUi {
        CardsUi {
            title: t,
            card: None,
        }
    }
    pub fn add_card(mut self, card: Card) -> CardsUi {
        self.card = Some(card);
        self
    }
    pub fn run(self) {
        use conrod::{Canvas, Colorable, Image, Positionable, Widget, color};
        use piston_window::{EventLoop, UpdateEvent};

        let mut window = window::setup(self.title);
        let mut ui = assets::conrod_ui(&mut window);
        let texture = match self.card {
            Some(card) => assets::card(&mut window, self.card.unwrap()),
            None => assets::hidden_card(&mut window),
        };

        window.set_ups(60);
        // Poll events from the window.
        while let Some(event) = window.next() {
            ui.handle_event(event.clone());
            window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
            event.update(|_| {
                ui.set_widgets(|mut ui| {
                    widget_ids!(CANVAS, RUST_LOGO);
                    Canvas::new().color(color::LIGHT_BLUE).set(CANVAS, &mut ui);
                    Image::from_texture(texture.clone())
                        .middle_of(CANVAS)
                        .set(RUST_LOGO, &mut ui);
                })
            });
        }
    }
}
