use macroquad::rand::gen_range;

use crate::{hero::Hero, sound_system::{SoundBox, SoundList}};
use super::attack::AttackType;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum State {
    Idle,
    Walk,
    Jump,
    AttackOne,
    AttackDouble,
    RepeatAttack,
    Dash,
    AirDash,
    Dying,
    Dead,
    Hit
}


impl Hero {


pub fn state_manager(&mut self, sound_bank: &SoundBox) {


    let previous_state = self.state;

    if self.hited {
        self.state = State::Hit;
        self.hitable = false;
        self.hited = false;
    }

    if self.health <= 0 && self.on_the_floor{
        self.state = State::Dying;
    }

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
            if self.sprite.current_frame ==  3 {
                sound_bank.play(SoundList::Sword1);
            }
            if self.sprite.current_frame == 11 {
                sound_bank.play(SoundList::Sword2);

            }
            self.velocity.x = 0.0;
            if self.sprite.is_animation_ended() {
                self.state = State::Idle;
                self.attack = None;
            }
        },
        
        State::AttackOne | State::RepeatAttack => {
            if self.sprite.current_frame == 0 {
                sound_bank.play(SoundList::Heavy);
            }
            if let Some(attack) = &self.attack {
                match attack {
                    AttackType::RepeatHeavy => self.state = State::RepeatAttack,
                    _ => {}
                }
                
            }
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
            if self.sprite.current_frame == 0 {
                let alea = gen_range(0, 3);
                match alea {
                    0 => sound_bank.play(SoundList::Huh1),
                    1 => sound_bank.play(SoundList::Huh2),
                    2 => sound_bank.play(SoundList::Huh3),
                    _ => {}

                }

                sound_bank.play(SoundList::Huh1)
            }
            if self.sprite.is_animation_ended() {
                self.state = State::Idle;
                self.hited = false;
                self.hitable = true;
            }
        },
        State::Dying => {
            if self.sprite.current_frame ==  0 {
                sound_bank.play(SoundList::Death);
            }
            if self.sprite.is_animation_ended() {self.state = State::Dead}
            self.hitable = false;
        }
        State::Dead => {
            self.hitable = false;
        },
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
}