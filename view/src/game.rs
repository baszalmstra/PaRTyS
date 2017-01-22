use piston_window::types::Color;
use cgmath::Vector2;

pub enum Shape {
    Circle,
    Rectangle,
    Moon,
    Triangle,
}

static CIRCLE: [[f64; 2]; 3] = [[-0.5, 0.5], [0.0, -0.5], [0.5, 0.5]];
static RECTANGLE: [[f64; 2]; 4] = [[-0.5, 0.5], [0.5, 0.5], [0.5, -0.5], [-0.5, -0.5]];
static MOON: [[f64; 2]; 3] = [[-0.5, 0.5], [0.0, -0.5], [0.5, 0.5]];
static TRIANGLE: [[f64; 2]; 3] = [[-0.7, 0.5], [0.0, -0.5], [0.7, 0.5]];

impl Shape {
    pub fn as_polygon(&self) -> &[[f64; 2]] {
        match *self {
            Shape::Circle => &CIRCLE,
            Shape::Rectangle => &RECTANGLE,
            Shape::Moon => &MOON,
            Shape::Triangle => &TRIANGLE,
        }
    }
}

pub struct Player {
    pub color: Color,
    pub base: Base,
    pub goblins: Vec<Goblin>,
    pub shape: Shape
}

impl Player {
    pub fn new(color: Color, position: Vector2<f32>, shape: Shape) -> Player {
        Player {
            color: color,
            base: Base::new(position),
            shape: shape,
            goblins: Vec::new()
        }
    }

    pub fn spawn_goblin(&mut self) -> bool {
        if(self.base.energy > 1.0) {
            self.goblins.push(Goblin::new(self.base.position));
            self.base.energy -= 1.0;
            return true
        } 

        false
    }
}

pub struct Base {
    pub position: Vector2<f32>,
    pub energy: f32
}

impl Base {
    pub fn new(position: Vector2<f32>) -> Base {
        Base {
            position: position,
            energy: 0.0
        }
    }
}

pub struct Goblin {
    pub position: Vector2<f32>
}

impl Goblin {
    pub fn new(position:Vector2<f32>) -> Goblin {
        Goblin {
            position: position
        }
    }
}

/// MapObject is an element that lives on the map.
trait MapObject {
    fn position(&self) -> Vector2<f32>;
}

impl MapObject for Base {
    fn position(&self) -> Vector2<f32> { self.position }
}

impl MapObject for Goblin {
    fn position(&self) -> Vector2<f32> { self.position }
}