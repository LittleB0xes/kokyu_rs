use macroquad::prelude::*;

use std::collections::HashMap;

use crate::sound_system::SoundBox;
use crate::sprite::{AnimationData, AnimatedSprite};
use crate::controls;
use crate::ghost::Ghost;
use attack::AttackType;

use self::attack::{get_hit_box, get_hit_point};
use state::State;

mod attack;
mod state;



pub struct Hero {
    pub position: Vec2,
    direction: f32,
    velocity: Vec2,
    state: State,
    animations: HashMap<State, AnimationData>,
    pub sprite: AnimatedSprite,

    health: i32,

    collision_box: Rect,

    on_the_floor: bool,
    hited: bool,
    hitable: bool,
    attack: Option<AttackType>
}

impl Hero {
    pub fn new(x: f32, y: f32, life_time: i32) -> Self {
        let position = Vec2 { x, y };
        let animations = HashMap::from([
            (State::AirDash, AnimationData{x: 0, y: 0, h: 64, w: 64, frames: 7, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::Dash, AnimationData{x: 0, y: 64, h: 64, w: 64, frames: 7, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::Walk, AnimationData{x: 0, y: 128, h: 64, w: 64, frames: 8, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::Idle, AnimationData{x: 0, y: 192, h: 64, w: 64, frames: 8, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::Jump, AnimationData{x: 0, y: 256, h: 64, w: 64, frames: 12, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::AttackDouble, AnimationData{x: 0, y: 384, h: 64, w: 64, frames: 19, speed: 2, pivot_x: 0, pivot_y: 0}),
            (State::AttackOne, AnimationData{x: 0, y: 448, h: 64, w: 64, frames: 17, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::RepeatAttack, AnimationData{x: 640, y: 448, h: 64, w: 64, frames: 7, speed: 4, pivot_x: 0, pivot_y: 0}),
            (State::Hit, AnimationData{x: 128, y: 512, h: 64, w: 64, frames: 5, speed: 2, pivot_x: 0, pivot_y: 0}),
            (State::Dying, AnimationData{x: 384, y: 320, h: 64, w: 64, frames: 13, speed: 10, pivot_x: 0, pivot_y: 0}),
            (State::Dead, AnimationData{x: 1152, y: 320, h: 64, w: 64, frames: 1, speed: 2, pivot_x: 0, pivot_y: 0}),
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

            health: 60 * life_time,

            on_the_floor: false,
            hited: false,
            hitable: true,
            attack: None,
        }
    }

    pub fn update(&mut self, monsters: &mut Vec<Ghost>, colliders: &Vec<Rect>, sound_bank: &SoundBox) {
        // Check monster collision

        self.hited = false;
        for monster in monsters.iter_mut() {

            if let Some(attack) = &self.attack {
                match self.get_hit_box(attack) {
                    Some(hbox) => {
                        if monster.is_hitable() && monster.get_collision_box(0.0, 0.0).overlaps(&hbox) {
                            monster.hit(get_hit_point(&attack));
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


        self.state_manager(sound_bank);

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
            else if let Some(AttackType::Heavy) = &self.attack {
                self.direction = 0.0;
                self.velocity.x = 0.0
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

            // Attack and combo management
            match &self.attack {
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
                Some(attack) => {
                    match attack {
                        AttackType::Heavy => {
                            if self.sprite.current_frame > 13 && is_key_pressed(KeyCode::C){
                                self.attack = Some(AttackType::RepeatHeavy)
                            }
                        }
                        _ => {}
                    }

                }
            }
        }
        else {
            self.velocity *= 0.7;
        }


        // Health
        self.health -= 1;
        match &self.attack {
            Some(_attack) => self.health -= 1,
            None => {}
        }
        if self.health < 0 {
            self.health = 0;
        }


        // Check collision with scene
        for collider in colliders.iter() {
            if self.get_collision_box(0.0, self.velocity.y).overlaps(collider){
                self.velocity.y = 0.0;
                self.on_the_floor = true;
            }
            if self.get_collision_box(self.velocity.x, 0.0).overlaps(collider){
                self.velocity.x = 0.0;
            }

        }
        // position update
        self.position += self.velocity;


        self.sprite.set_position_to(self.position);
    }


    pub fn get_health(&self) -> i32 {
        self.health
        
    }

    pub fn is_dead(&self) -> bool {
        match self.state {
            State::Dead => true,
            _ => false
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