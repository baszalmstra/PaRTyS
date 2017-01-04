extern crate piston_window;
extern crate opengl_graphics;
extern crate cgmath;
extern crate num_traits;

use piston_window::*;
use opengl_graphics::GlGraphics;

mod app;
mod state;
mod player;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("PaRTyS", [1920, 1080])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = app::App::new();

    app.load();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    while let Some(e) = window.next() {
        if let Some(ref args) = e.render_args() {
            app.render(args, &mut gl);
        }

        if let Some(ref args) = e.update_args() {
            app.update(args);
        }

        // if let Some(ref args) = e.press_args() {
        //     //app.key_press
        // }
    }
}