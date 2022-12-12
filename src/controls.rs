use macroquad::{input::KeyCode, prelude::is_key_down};


pub fn get_x_axis() -> f32 {
    let left = if is_key_down(KeyCode::Left) {1.0} else {0.0};
    let right = if is_key_down(KeyCode::Right) {1.0} else {0.0};

    right - left
}