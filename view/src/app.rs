use piston_window::*;
use opengl_graphics::GlGraphics;
use cgmath::Vector2;

use state;
use game::*;
use num_traits::Float;
use time;

pub struct App {
    state: state::GameState,
    lastUpdateTime: Option<f32>
}

impl App {
    pub fn new() -> App {
        App { 
            state: state::GameState::new(),
            lastUpdateTime: None
        }
    }

    pub fn load(&mut self) {
        // Create two players
        self.state.players.push(Player::new([1.0, 0.0, 0.0, 1.0],
                                            Vector2::new(30.0, 400.0),
                                            Shape::Triangle));

        self.state.players.push(Player::new([1.0, 1.0, 0.0, 1.0],
                                            Vector2::new(1250.0, 400.0),
                                            Shape::Rectangle));
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

        let window_background = [0.1, 0.1, 0.1, 1.0];

        // Start drawing the viewport
        gl.draw(args.viewport(), |_, gl| {

            // Clear to black
            clear(window_background, gl);

            // Draw player base
            for player in self.state.players.iter() {
                polygon(player.color,
                        player.shape.as_polygon(),
                        c.transform
                            .trans(player.base.position.x as f64, player.base.position.y as f64)
                            .scale(20.0, 20.0),
                        gl);

                for goblin in player.goblins.iter() {
                    polygon(player.color,
                            player.shape.as_polygon(),
                            c.transform
                                .trans(goblin.position.x as f64, goblin.position.y as f64)
                                .scale(8.0, 8.0),
                            gl);
                }
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let currentTime = (time::precise_time_ns() as f64 * 1e-9) as f32;
        let delta = match self.lastUpdateTime {
            Some(val) => currentTime - val,
            None => 0.0
        };
        self.lastUpdateTime = Some(currentTime);

        // Spawn goblins
        for mut player in &mut self.state.players {
            player.base.energy += delta;
            player.spawn_goblin();
        }

        // Move goblins 
        for mut player in &mut self.state.players {
            for mut goblin in &mut player.goblins {
                goblin.position.x += delta * 30.0;
            }
        }
    }
}