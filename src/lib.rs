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
    title: &'static str,
}

impl CardsUi {
    pub fn new(t: &'static str) -> CardsUi {
        CardsUi { title: t }
    }
    pub fn run(self, mut app: &mut CardsApp) {
        use piston_window::{EventLoop, UpdateEvent};

        let mut window = window::setup(self.title);
        let mut ui = assets::conrod_ui(&mut window);

        window.set_ups(60);
        // Poll events from the window.
        while let Some(event) = window.next() {
            ui.handle_event(event.clone());
            window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
            event.update(|_| ui.set_widgets(|mut ui| set_widgets(&mut ui, &mut app, &mut window)));
        }
    }
}

use piston_window::PistonWindow;
fn set_widgets(ui: &mut backend::UiCell, app: &mut CardsApp, window: &mut PistonWindow) {
    use conrod::*;//{Button, Canvas, Colorable, Image, Positionable, Widget, color};
    Canvas::new().color(color::LIGHT_BLUE).set(CANVAS, ui);

    let card = app.last_card();
    let texture = match card {
        Some(card) => assets::card(window, card),
        None => assets::hidden_card(window),
    };

    Image::from_texture(texture)
        .middle_of(CANVAS)
        .set(CARD, ui);

    Button::new()
        .rgb(0.4, 0.75, 0.6)
        .mid_left_of(CANVAS)
        .react(|| {
            let c = app.last_card();
            println!("Card was: {}", c.unwrap());
            println!("Button was pressed");
            app.add_card(Card::new(Value::Queen, Suit::Hearts));
        })
        .set(BUTTON, ui);
}

widget_ids! {
    CANVAS,
    BUTTON,
    CARD,
}
