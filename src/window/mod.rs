extern crate piston_window;

pub fn setup(title: &'static str) -> piston_window::PistonWindow {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = piston_window::OpenGL::V3_2;

    // Construct the window.
    piston_window::WindowSettings::new(title, [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .vsync(true)
        .samples(4)
        .build()
        .unwrap()
}
