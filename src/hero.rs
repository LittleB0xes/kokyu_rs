use macroquad::prelude::*;

use std::collections::HashMap;

use crate::sprite::{AnimationData, AnimatedSprite};
use crate::controls;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
enum State {
    Idle,
    Walk,
    Jump,
    AttackOne,
    AttackDouble,
    RepeatAttack,
    Dash,
    AirDash,
    Dead,
}

enum AttackType {
    Heavy,
    Double,
    AttackDash{timer: i32, dir: f32},
}

pub struct Hero {
    pub position: Vec2,
    direction: f32,
    velocity: Vec2,
    state: State,
    animations: HashMap<State, AnimationData>,
    pub sprite: AnimatedSprite,

    collision_box: Rect,

    on_the_floor: bool,
    dash: bool,
    attack: Option<AttackType>
}

impl Hero {
    pub fn new(x: f32, y: f32) -> Self {
        let position = Vec2 { x, y };
        let animations = HashMap::from([
            (State::AirDash, AnimationData{x: 0, y: 0, h: 64, w: 64, frames: 7, speed: 4, pivot_x: 0, pivot_y: 48}),
            (State::Dash, AnimationData{x: 0, y: 64, h: 64, w: 64, frames: 7, speed: 4, pivot_x: 0, pivot_y: 48}),
            (State::Walk, AnimationData{x: 0, y: 128, h: 64, w: 64, frames: 8, speed: 4, pivot_x: 0, pivot_y: 48}),
            (State::Idle, AnimationData{x: 0, y: 192, h: 64, w: 64, frames: 8, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::Jump, AnimationData{x: 0, y: 256, h: 64, w: 64, frames: 12, speed: 4, pivot_x: 0, pivot_y: 48}),
            (State::Jump, AnimationData{x: 0, y: 256, h: 64, w: 64, frames: 12, speed: 4, pivot_x: 0, pivot_y: 48}),
            (State::AttackDouble, AnimationData{x: 0, y: 384, h: 64, w: 64, frames: 19, speed: 1, pivot_x: 0, pivot_y: 48}),
            (State::AttackOne, AnimationData{x: 0, y: 448, h: 64, w: 64, frames: 17, speed: 4, pivot_x: 0, pivot_y: 48}),
        ]);

        let state = State::Idle;
        let mut sprite = AnimatedSprite::new(animations.get(&state).expect("No animation"));
        sprite.set_position_to(position);


        Self {
            position,
            direction: 0.0,
            velocity: Vec2::ZERO,
            state,
            animations,
            sprite,
            collision_box: Rect { x: 27.0, y: 28.0, w: 10.0, h: 20.0 },

            on_the_floor: false,
            dash: false,
            attack: None,
        }
    }

    pub fn update(&mut self) {
        // Gravity
        self.velocity.y += 0.5;

        self.direction = controls::get_x_axis();

        if let Some(AttackType::AttackDash { timer: _, dir }) = &self.attack {
            self.direction = *dir;
            self.velocity.x = self.direction * 6.0;
        }

        else if self.direction != 0.0 {
            self.velocity.x = self.direction * 2.0;
        }
        else {
            self.velocity.x *= 0.8;
        }


        if self.on_the_floor && is_key_pressed(KeyCode::Space) {
            // Jump
            self.velocity.y = -8.0;
            self.on_the_floor = false;
        }

        match self.attack {
            None => {
                if is_key_pressed(KeyCode::V) {
                    if self.direction != 0.0 && self.on_the_floor {
                        self.attack = Some(AttackType::AttackDash{timer: 10, dir: self.direction});
                    }
                    else {
                        self.attack = Some(AttackType::Double);
                    }
                }
                if is_key_pressed(KeyCode::C) {self.attack = Some(AttackType::Heavy)}
            },
            Some(_) => {}
        }


        self.state_manager();
        if self.get_collision_box(0.0, self.velocity.y).overlaps(&Rect{x: 0.0, y: 101.0, w: 426.0, h: 16.0}){
            self.velocity.y = 0.0;
            self.on_the_floor = true;
        }
        // position update
        self.position += self.velocity;


        self.sprite.set_position_to(self.position);
    }


    fn state_manager(&mut self) {

        let previous_state = self.state;
        match self.state {
            State::Idle => {
                if self.direction != 0.0 {
                    self.state = State::Walk;
                }

                if !self.on_the_floor {
                    self.state = State::Jump;
                };
                match &self.attack {
                    Some(a) => {
                        match a {
                            AttackType::Double => {self.state = State::AttackDouble},
                            AttackType::Heavy => {self.state = State::AttackOne},
                            _ => {}
                        }
                    },
                    None => {}
                };
            },
            State::Walk => {
                if self.direction == 0.0 {self.state = State::Idle;}
                if !self.on_the_floor {self.state = State::Jump;}
                match &self.attack {
                    Some(a) => {
                        match a {
                            AttackType::Double => {self.state = State::AttackDouble},
                            AttackType::Heavy => {self.state = State::AttackOne},
                            AttackType::AttackDash{timer: _, dir: _} => {
                                self.state = State::Dash;
                            }
                            _ => {}
                        }
                    },
                    None => {}
                };
            },
            State::Jump => {
                if self.on_the_floor {self.state = State::Idle}
                match &self.attack {
                    Some(a) => {
                        match a {
                            AttackType::Double => {self.state = State::AttackDouble},
                            _ => {}
                        }
                    },
                    None => {}
                };
            },

            State::AttackDouble => {
                self.velocity.x = 0.0;
                if self.sprite.is_animation_ended() {
                    self.state = State::Idle;
                    self.attack = None;
                }
            },
            
            State::AttackOne => {
                self.velocity.x = 0.0;
                if self.sprite.is_animation_ended() {
                    self.state = State::Idle;
                    self.attack = None;
                }
            },

            State::Dash => {
                if let Some(AttackType::AttackDash{timer, dir }) = &self.attack {
                    let t = timer - 1;
                    if t > 0 {
                        self.attack = Some(AttackType::AttackDash { timer: t , dir: *dir});

                    }
                    else {
                        self.attack = None;
                        self.state = State::Idle;
                    }
                }

            }

            _ => {}
        }

        if self.direction == 1.0 {
            self.sprite.flip_x = false;
        }
        else if self.direction == -1.0 {
            self.sprite.flip_x = true;
        }

        if previous_state != self.state {
            self.sprite.set_animation(self.animations.get(&self.state).expect("No animation"));
            self.sprite.play();
        }
    }


    pub fn get_collision_box(&self, dx: f32, dy: f32) -> Rect {
        Rect { x: self.position.x + self.collision_box.x + dx, y: self.position.y + self.collision_box.y + dy, w: self.collision_box.w, h: self.collision_box.h }
    }
}