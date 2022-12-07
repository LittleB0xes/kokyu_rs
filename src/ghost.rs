use std::collections::HashMap;

use macroquad::prelude::*;

use crate::sprite::{AnimatedSprite, AnimationData};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum MonsterState {
    Idle,
    Birth,
    Death,
    Hit,

}

pub struct Ghost {
    pub position: Vec2,
    pub sprite: AnimatedSprite,
    collision_box: Rect,
    state: MonsterState,
    animations: HashMap<MonsterState, AnimationData>,
    velocity: Vec2,

    
}

impl Ghost {
    pub fn new(x: f32, y: f32) -> Self {
        let position = Vec2{x, y};
        let animations = HashMap::from([
            (MonsterState::Idle, AnimationData{x: 0, y: 0, h: 64, w: 64, frames: 5, speed: 8, pivot_x: 0, pivot_y: 0}),
            (MonsterState::Hit, AnimationData{x: 0, y: 64, h: 64, w: 64, frames: 10, speed: 4, pivot_x: 0, pivot_y: 0}),
            (MonsterState::Death, AnimationData{x: 0, y: 128, h: 64, w: 64, frames: 10, speed: 8, pivot_x: 0, pivot_y: 0}),
            (MonsterState::Birth, AnimationData{x: 0, y: 192, h: 64, w: 64, frames: 13, speed: 8, pivot_x: 0, pivot_y: 0}),


        ]);

        let state = MonsterState::Idle;
        let mut sprite = AnimatedSprite::new(animations.get(&state).expect("No animation in library"));
        sprite.set_position_to(position);

        Self {
            position,
            velocity: Vec2::ZERO,
            state,
            animations,
            sprite,
            collision_box: Rect { x: 26.0, y: 19.0, w: 14.0, h: 22.0 },
        }

    }

    pub fn update(&mut self, hero_pos: Vec2) {

        // Look in the right direction
        if self.position.x > hero_pos.x {
            self.sprite.flip_x = true;
        }
        else if self.position.x < hero_pos.x {
            self.sprite.flip_x = false;
        }
    }
    
    pub fn get_collision_box(&self, dx: f32, dy: f32) -> Rect {
        Rect { x: self.position.x + self.collision_box.x + dx, y: self.position.y + self.collision_box.y + dy, w: self.collision_box.w, h: self.collision_box.h }
    }
}