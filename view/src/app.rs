use piston_window::*;
use opengl_graphics::GlGraphics;
use cgmath::Vector2;

use state;
use player::*;
use num_traits::Float;

pub struct App {
    state: state::State,
}

impl App {
    pub fn new() -> App {
        App { state: state::State::new() }
    }

    pub fn load(&mut self) {
        self.state.players.push(Player::new([1.0, 0.0, 0.0, 1.0],
                                            Vector2::new(10.0, 10.0),
                                            Shape::Triangle));
    }

    fn create_context(&mut self, args: &RenderArgs) -> Context {

        // Make sure this fits with the aspect ratio
        let width = self.state.width as f64;
        let height = self.state.height as f64;

        let x_scale = width / args.width as f64;
        let y_scale = height / args.height as f64;

        let scale = y_scale.max(x_scale);

        if y_scale < scale {
            Context::new_abs(width, args.height as f64 * scale)
                .trans(0.0, (args.height as f64 * scale - height) * 0.5)
        } else {
            Context::new_abs(args.width as f64 * scale, height)
                .trans((args.width as f64 * scale - width) * 0.5, 0.0)
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {

        let c: Context = self.create_context(args);

        let window_background = [0.0; 4];
        let triangle = self.state.players[0].shape.as_polygon();


        gl.draw(args.viewport(), |_, gl| {
            clear(window_background, gl);

            for player in self.state.players.iter() {
                polygon([1.0, 0.0, 0.0, 1.0],
                        player.shape.as_polygon(),
                        c.transform
                            .trans(player.position.x as f64, player.position.y as f64)
                            .scale(100.0, 100.0),
                        gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {}
}