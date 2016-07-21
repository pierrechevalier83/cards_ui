#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

// A simple demonstration of how to instantiate an `Image` widget.

#[macro_use]
extern crate conrod;
extern crate piston_window;

mod assets;

pub fn run() {
    use conrod::{Canvas, Colorable, Image, Positionable, Theme, Widget, color};
    use piston_window::{EventLoop, Flip, PistonWindow, Texture, UpdateEvent, WindowSettings};
    use std::sync::Arc;

    // Conrod is backend agnostic. Here, we define the `piston_window` backend to use for our `Ui`.
    type Backend = (piston_window::G2dTexture<'static>, piston_window::Glyphs);
    type Ui = conrod::Ui<Backend>;

    // Change this to OpenGL::V2_1 if not working.
    let opengl = piston_window::OpenGL::V3_2;

    // Construct the window.
    let mut window: PistonWindow = WindowSettings::new("Image Widget Demonstration", [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .vsync(true)
        .samples(4)
        .build()
        .unwrap();


    // construct our `Ui`.
    let mut ui = {
        let font_path = assets::assets_folder().join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = piston_window::Glyphs::new(&font_path, window.factory.clone()).unwrap();
        Ui::new(glyph_cache, theme)
    };

    // The texture to use for the `Image`.
    let rust_logo = assets::rust_logo(&mut window);

    window.set_ups(60);

    // Poll events from the window.
    while let Some(event) = window.next() {
        ui.handle_event(event.clone());
        window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
        event.update(|_| {
            ui.set_widgets(|mut ui| {
                widget_ids!(CANVAS, RUST_LOGO);
                Canvas::new().color(color::LIGHT_BLUE).set(CANVAS, &mut ui);
                Image::from_texture(rust_logo.clone())
                    .middle_of(CANVAS)
                    .set(RUST_LOGO, &mut ui);
            })
        });
    }
}
