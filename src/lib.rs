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
pub mod window;
mod cards_app;

pub use cards::card::{Card, Value, Suit};
pub use cards_app::CardsApp;

pub struct CardsUi {
}

impl CardsUi {
    pub fn new() -> CardsUi {
        CardsUi {}
    }
    pub fn run(self, mut window: &mut PistonWindow, mut app: &mut CardsApp) {
        use piston_window::{EventLoop, UpdateEvent};

        let mut ui = assets::conrod_ui(&mut window);

        window.set_ups(60);
        // Poll events from the window.
        while let Some(event) = window.next() {
            ui.handle_event(event.clone());
            window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
            event.update(|_| ui.set_widgets(|mut ui| set_widgets(&mut ui, &mut app)));
        }
    }
}

use piston_window::PistonWindow;
fn set_widgets(ui: &mut backend::UiCell, app: &mut CardsApp) {
    use conrod::*;//{Button, Canvas, Colorable, Image, Positionable, Widget, color};
    Canvas::new().color(color::LIGHT_BLUE).set(CANVAS, ui);

    Image::from_texture(app.card_texture())
        .middle_of(CANVAS)
        .set(CARD, ui);

    Button::new()
        .rgb(0.4, 0.75, 0.6)
        .mid_left_of(CANVAS)
        .react(|| {
            app.add_card(Card::new(Value::Queen, Suit::Hearts));
        })
        .set(BUTTON, ui);
}

widget_ids! {
    CANVAS,
    BUTTON,
    CARD,
}
