use std::collections::HashMap;

use macroquad::{prelude::*, rand::gen_range};

use crate::sprite::{AnimatedSprite, AnimationData};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum MonsterState {
    Idle,
    Birth,
    Dead,
    Hit,
}

enum Behaviour {
    UpDown {yo: f32, speed: f32, dt: f32},
    StandBy,
}

pub struct Ghost {
    pub position: Vec2,
    pub velocity: Vec2,
    pub sprite: AnimatedSprite,
    direction: f32,
    collision_box: Rect,
    state: MonsterState,
    animations: HashMap<MonsterState, AnimationData>,

    behaviour: Behaviour,

    health: i32,

    hitable: bool,
    active: bool,
    hited: bool,

    
}

impl Ghost {
    pub fn new(x: f32, y: f32) -> Self {
        let position = Vec2{x, y};
        let animations = HashMap::from([
            (MonsterState::Idle, AnimationData{x: 0, y: 0, h: 64, w: 64, frames: 5, speed: 8, pivot_x: 0, pivot_y: 0}),
            (MonsterState::Hit, AnimationData{x: 0, y: 64, h: 64, w: 64, frames: 10, speed: 4, pivot_x: 0, pivot_y: 0}),
            (MonsterState::Dead, AnimationData{x: 0, y: 128, h: 64, w: 64, frames: 10, speed: 8, pivot_x: 0, pivot_y: 0}),
            (MonsterState::Birth, AnimationData{x: 0, y: 192, h: 64, w: 64, frames: 13, speed: 8, pivot_x: 0, pivot_y: 0}),


        ]);

        let state = MonsterState::Birth;
        let mut sprite = AnimatedSprite::new(animations.get(&state).expect("No animation in library"));
        sprite.set_position_to(position);

        Self {
            position,
            velocity: Vec2::ZERO,
            state,
            animations,
            sprite,
            collision_box: Rect { x: 25.0, y: 19.0, w: 15.0, h: 22.0 },
            direction: 0.0,

            behaviour: Behaviour::StandBy,

            health: 3,

            hitable:false,
            hited: false,

            active: true,
        }

    }

    pub fn update(&mut self, hero_pos: Vec2) {

        self.brain(hero_pos);


        // Look in the right direction
        if self.position.x > hero_pos.x {
            self.direction = -1.0;
        }
        else if self.position.x < hero_pos.x {
            self.direction = 1.0;
        }

        self.state_manager();

        self.position += self.velocity;
        self.sprite.set_position_to(self.position);
    }


    fn brain(&mut self, _hero_pos: Vec2) {
        if self.state == MonsterState::Idle {
            match self.behaviour {
                Behaviour::StandBy => {
                    if gen_range(0, 100) < 2 {
                        self.behaviour = Behaviour::UpDown { yo: self.position.y, speed: 0.01, dt: 0.0 };
                    }
                },
                Behaviour::UpDown { yo, speed, dt } => {
                    self.position.y = yo + 15.0 * dt.sin();
                    self.behaviour = Behaviour::UpDown { yo, speed: 0.01, dt: dt + speed };
                }
            }

        } 
        
    }

    fn state_manager(&mut self) {
        let previous_state = self.state;
         match self.state {
            MonsterState::Birth => {
                if self.sprite.is_animation_ended() {
                    self.state = MonsterState::Idle;
                    self.hitable = true;
                }
            },
            MonsterState::Idle => {},
            MonsterState::Hit => {
                self.hitable = false;
                self.hited = false;
                if self.sprite.is_animation_ended() {
                    self.state = MonsterState::Idle;
                    self.hitable = true;

                    if self.health <= 0 {
                        self.state = MonsterState::Dead;
                        self.hitable = false;
                    }
                }
            },
            MonsterState::Dead => {
                self.hitable = false;
                if self.sprite.is_animation_ended() {
                    self.active = false;
                }
            }

        }
        if self.direction == 1.0 {
            self.sprite.flip_x = false;
        }
        else if self.direction == -1.0 {
            self.sprite.flip_x = true;
        }


        //if self.health <= 0 {self.state = MonsterState::Dead}
        if self.hited {self.state = MonsterState::Hit}


        if previous_state != self.state {
            self.sprite.set_animation(self.animations.get(&self.state).expect("No animation"));
            self.sprite.play();
        }
    }
    
    pub fn get_collision_box(&self, dx: f32, dy: f32) -> Rect {
        Rect { x: self.position.x + self.collision_box.x + dx, y: self.position.y + self.collision_box.y + dy, w: self.collision_box.w, h: self.collision_box.h }
    }

    pub fn hit(&mut self, value: i32) {
        self.hited = true;
        self.hitable = false;
        self.health -= value;
    }
    pub fn is_hitable(&self) -> bool {
        self.hitable
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}