use macroquad::prelude::*;
use game:: Game;

mod game;
mod hero;
mod sprite;
mod particle;
mod light;
mod controls;


// Entry point for macroquad is the window_conf function
#[macroquad::main(window_conf())]



async fn main() {
    let mut game = Game::new();
    loop {
        game.update();
        game.render();

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Kokyu".to_owned(),
        window_width: 1278,
        window_height: 720,
        fullscreen: false,
        //high_dpi: true,
        ..Default::default()
    }
}

