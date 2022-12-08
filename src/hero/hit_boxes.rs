use macroquad::prelude::Rect;

#[derive(Debug)]
pub enum AttackType {
    Heavy,
    Double,
    RepeatHeavy,
    AttackDash{timer: i32, dir: f32},
    AttackAirDash{timer: i32, dir: f32},
}

pub fn get_hit_point(attack: &AttackType) -> i32 {
    match attack {
        AttackType::Double | AttackType::AttackAirDash { timer: _, dir: _ } | AttackType::AttackDash { timer: _, dir: _} => 1,
        AttackType::Heavy => 2,
        AttackType::RepeatHeavy => 2,
    }
}

pub fn get_hit_box(attack: &AttackType, frame: i32, flip_x: bool) -> Option<Rect> {
    let h_box = match attack {
        AttackType::Double => {
            match frame {
                6 | 7 | 8 | 9 => Some(Rect{x: 41.0, y:31.0, w: 16.0, h: 16.0 }),
                13 | 14 | 15 => Some(Rect{x: 9.0, y:31.0, w: 16.0, h: 16.0 }),
                _ => None
            }
        },
        AttackType::Heavy => {
            match frame {
                12|13|14 => Some(Rect { x: 34.0, y: 4.0, w: 27.0, h: 44.0 }),
                _ => None,
            }
        },
        AttackType::RepeatHeavy => {
            match frame {
                12|13|14 => Some(Rect { x: 34.0, y: 4.0, w: 27.0, h: 44.0 }),
                _ => None,
            }
        },
        AttackType::AttackDash {timer: _, dir: _ } | &AttackType::AttackAirDash {timer: _, dir: _ }=> {
            Some(Rect { x: 36.0, y: 32.0, w: 11.0, h: 14.0 })
        },
    };


    // Make symetric box if sprite is flip
    match h_box {
        Some(b) => {
            if flip_x {
                Some(Rect { x:64.0 - b.x - b.w, y: b.y, w: b.w, h: b.h })
            }
            else {
                Some(b)
            }
        }
        _ => None
    }
}