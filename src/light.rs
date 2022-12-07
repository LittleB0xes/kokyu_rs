use macroquad::{prelude::*, rand::gen_range};

pub struct Light {
    position: Vec2,
    pub color: Color,
    dt: f32,
    radius: f32,
}

impl Light {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        let color = WHITE;
        Self {
            position: Vec2 { x, y},
            color,
            dt: gen_range(0.0, 1.5),
            radius,
        }
    }

    pub fn update(&mut self) {
        self.dt += 0.01;
        self.color.a = 0.10 + 0.05 * (0.5 * self.dt).sin();

    }
    pub fn get_radius(&self) -> f32 {
        self.radius + 2.0 * self.dt.sin()
    }
    pub fn get_position(&self) -> Vec2 {
        Vec2 {
            x: self.position.x -  self.get_radius() + 2.0 * self.dt.cos(),
            y: self.position.y -  self.get_radius() + 2.0 * self.dt.sin(),
        }

    }

}