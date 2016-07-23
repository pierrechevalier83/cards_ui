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
pub use cards_app::CardsApp;

pub struct CardsUi {
    // TODO: layout
    title: &'static str,
}

impl CardsUi {
    pub fn new(t: &'static str) -> CardsUi {
        CardsUi { title: t }
    }
    pub fn run(self, app: CardsApp) {
        use conrod::{Canvas, Colorable, Image, Positionable, Widget, color};
        use piston_window::{EventLoop, UpdateEvent};

        let mut window = window::setup(self.title);
        let mut ui = assets::conrod_ui(&mut window);
        let card: Option<Card> = app.last_card();
        let texture = match card {
            Some(card) => assets::card(&mut window, card),
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
