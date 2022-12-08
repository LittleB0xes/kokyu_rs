use macroquad::prelude::*;

use std::collections::HashMap;

use crate::sprite::{AnimationData, AnimatedSprite};
use crate::controls;
use crate::ghost::Ghost;
use hit_boxes::AttackType;

use self::hit_boxes::get_hit_box;

mod hit_boxes;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
enum State {
    Idle,
    Walk,
    Jump,
    AttackOne,
    AttackDouble,
    //RepeatAttack,
    Dash,
    AirDash,
    Dead,
    Hit
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
    hited: bool,
    attack: Option<AttackType>
}

impl Hero {
    pub fn new(x: f32, y: f32) -> Self {
        let position = Vec2 { x, y };
        let animations = HashMap::from([
            (State::AirDash, AnimationData{x: 0, y: 0, h: 64, w: 64, frames: 7, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::Dash, AnimationData{x: 0, y: 64, h: 64, w: 64, frames: 7, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::Walk, AnimationData{x: 0, y: 128, h: 64, w: 64, frames: 8, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::Idle, AnimationData{x: 0, y: 192, h: 64, w: 64, frames: 8, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::Jump, AnimationData{x: 0, y: 256, h: 64, w: 64, frames: 12, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::Jump, AnimationData{x: 0, y: 256, h: 64, w: 64, frames: 12, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::AttackDouble, AnimationData{x: 0, y: 384, h: 64, w: 64, frames: 19, speed: 1, pivot_x: 0, pivot_y: 0}),
            (State::AttackOne, AnimationData{x: 0, y: 448, h: 64, w: 64, frames: 17, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::Hit, AnimationData{x: 128, y: 512, h: 64, w: 64, frames: 5, speed: 2, pivot_x: 0, pivot_y: 0}),
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
            hited: false,
            attack: None,
        }
    }

    pub fn update(&mut self, monsters: &mut Vec<Ghost>) {
        // Check monster collision

        self.hited = false;
        for monster in monsters.iter_mut() {

            if let Some(attack) = &self.attack {
                match self.get_hit_box(attack) {
                    Some(hbox) => {
                        if monster.is_hitable() && monster.get_collision_box(0.0, 0.0).overlaps(&hbox) {
                            println!("Hit Monster");
                            monster.hit();
                        }

                    },
                    None => {}
                }

            }

            // Check body to body collision
            else if monster.is_hitable() && self.state != State::Hit && self.get_collision_box(0.0, 0.0).overlaps(&monster.get_collision_box(0.0, 0.0)) {
                self.hited = true;
                let bump_dir = (monster.position - self.position).normalize();
                self.velocity = -8.0 * bump_dir;
            }


        }


        self.state_manager();

        // Gravity
        self.velocity.y += 0.5;

        if self.state != State::Hit {
            self.direction = controls::get_x_axis();

            if let Some(AttackType::AttackDash { timer: _, dir }) = &self.attack {
                self.direction = *dir;
                self.velocity.x = self.direction * 6.0;
            }
            else if let Some(AttackType::AttackAirDash { timer: _, dir }) = &self.attack {
                self.direction = *dir;
                self.velocity.x = self.direction * 8.0;
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
                        else if self.direction != 0.0 && !self.on_the_floor {
                            self.attack = Some(AttackType::AttackAirDash{timer: 10, dir: self.direction});

                        }
                        else {
                            self.attack = Some(AttackType::Double);
                        }
                    }
                    if is_key_pressed(KeyCode::C) {self.attack = Some(AttackType::Heavy)}
                },
                Some(_) => {}
            }
        }
        else {
            self.velocity *= 0.7;
        }



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

        if self.hited {self.state = State::Hit}

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
                            AttackType::AttackAirDash { timer: _, dir: _ } => {self.state = State::AirDash},
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
            },

            State::AirDash => {
                if self.on_the_floor {
                    self.state = State::Idle;
                    self.attack = None;
                }
                if let Some(AttackType::AttackAirDash{timer, dir }) = &self.attack {
                    let t = timer - 1;
                    if t > 0 {
                        self.attack = Some(AttackType::AttackAirDash { timer: t , dir: *dir});

                    }
                    else {
                        self.attack = None;
                        self.state = State::Walk;
                    }
                }

            },

            State::Hit => {
                self.state = State::Hit;
                if self.sprite.is_animation_ended() {
                    self.state = State::Idle;
                    self.hited = false;
                }
            },
            State::Dead => {},
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

    pub fn get_hit_box(&self, attack: &AttackType) -> Option<Rect> {
        match get_hit_box(&attack, self.sprite.current_frame, self.sprite.flip_x) {
            Some(hbox) => {
                Some(Rect{
                    x: hbox.x + self.position.x,
                    y: hbox.y + self.position.y,
                    w: hbox.w,
                    h: hbox.h
                })
            },
            _ => None
        }
    }

    pub fn debug_hitbox(&self) {
        if let Some(attack) = &self.attack {
            if let Some(h_box) = get_hit_box(attack, self.sprite.current_frame, self.sprite.flip_x) {
                draw_rectangle_lines(h_box.x + self.position.x, h_box.y + self.position.y, h_box.w, h_box.h, 1.0, YELLOW);
            }
        }
    }
}