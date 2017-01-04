use piston_window::types::Color;
use cgmath::Vector2;

pub enum Shape {
    Circle,
    Rectangle,
    Moon,
    Triangle,
}

static CIRCLE: [[f64; 2]; 3] = [[-0.5, 0.5], [0.0, -0.5], [0.5, 0.5]];
static RECTANGLE: [[f64; 2]; 3] = [[-0.5, 0.5], [0.0, -0.5], [0.5, 0.5]];
static MOON: [[f64; 2]; 3] = [[-0.5, 0.5], [0.0, -0.5], [0.5, 0.5]];
static TRIANGLE: [[f64; 2]; 3] = [[-0.5, 0.5], [0.0, -0.5], [0.5, 0.5]];

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
    pub position: Vector2<f32>,
    pub shape: Shape,
}

impl Player {
    pub fn new(color: Color, position: Vector2<f32>, shape: Shape) -> Player {
        Player {
            color: color,
            position: position,
            shape: shape,
        }
    }
}