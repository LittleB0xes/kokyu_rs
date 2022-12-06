use macroquad::{input::KeyCode, prelude::is_key_down};
use std::collections::HashMap;

enum Command {
    Left,
    Right,
    Up,
    Down,
    A
}

pub fn get_x_axis() -> f32 {
    let left = if is_key_down(KeyCode::Left) {1.0} else {0.0};
    let right = if is_key_down(KeyCode::Right) {1.0} else {0.0};

    right - left
}
pub fn get_y_axis() -> f32 {
    let up = if is_key_down(KeyCode::Up) {1.0} else {0.0};
    let down= if is_key_down(KeyCode::Down) {1.0} else {0.0};

    down - up
}