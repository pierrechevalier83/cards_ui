#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

#[macro_use]
extern crate conrod;
extern crate piston_window;

mod backend;
mod assets;
mod window;

pub fn run() {
    use conrod::{Canvas, Colorable, Image, Positionable, Widget, color};
    use piston_window::{EventLoop, UpdateEvent};

    let mut window = window::setup("Hidden Card");
    let mut ui = assets::conrod_ui(&mut window);
    let hidden_card = assets::hidden_card(&mut window);

    window.set_ups(60);
    // Poll events from the window.
    while let Some(event) = window.next() {
        ui.handle_event(event.clone());
        window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
        event.update(|_| {
            ui.set_widgets(|mut ui| {
                widget_ids!(CANVAS, RUST_LOGO);
                Canvas::new().color(color::LIGHT_BLUE).set(CANVAS, &mut ui);
                Image::from_texture(hidden_card.clone())
                    .middle_of(CANVAS)
                    .set(RUST_LOGO, &mut ui);
            })
        });
    }
}
