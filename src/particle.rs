use crate::sprite::{AnimatedSprite, AnimationData};
use macroquad::{prelude::*, rand::gen_range};
pub struct Particle {
    pub sprite: AnimatedSprite,
    alpha: Vec<f32>,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        let mut sprite = AnimatedSprite::new(&AnimationData{x: 0, y: 0, w: 16, h: 16, speed: 10, frames: 60, pivot_x: 0, pivot_y:0});
        sprite.set_position_to(Vec2{x, y});

        // Starting frame randomization
        sprite.set_frame(gen_range(0, sprite.frames));
        // Randomization of transparency
        let mut alpha = Vec::new();
        for _i in 0..sprite.frames as usize {
            alpha.push(gen_range(30.0, 55.0) / 255.0);
        }
        Self {
            sprite,
            alpha,
        }
    }

    pub fn update(&mut self) {
        // only update the visual effect (id transparency), not the animation
        // Animation is already updated when rendering (AnimatedSprite)
        let frame = self.sprite.current_frame as usize;
        self.sprite.set_transparency(self.alpha[frame]);

    }
}