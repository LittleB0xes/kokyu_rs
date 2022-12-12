use macroquad::prelude::*;
use super::Game;

use super::GameState;
use super::TextureName;


impl Game {

    pub fn render_title_screen(&mut self, screen: GameState) {
        self.render_background();
        self.render_particles();
        self.render_ground_mask();
        self.render_letterbox_mask();
        // Render title
       
        
        match screen {
            GameState::Intro => {
                let title_params = DrawTextureParams {
                    dest_size: Some(Vec2 { x: 128.0, y: 64.0 }),
                    source: Some(Rect::new(0.0, 0.0, 128.0, 64.0)),
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                };

                draw_texture_ex(self.get_texture(TextureName::Title), 149.0, 8.0, WHITE, title_params);
                if self.transition_alpha <= 0.2 {
                    let press_params = DrawTextureParams {
                        dest_size: Some(Vec2 { x: 128.0, y: 16.0 }),
                        source: Some(Rect::new(0.0, 64.0, 128.0, 16.0)),
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    };
                    let color = Color::new(1.0, 1.0, 1.0, 0.9 + 0.1 * ((get_time() * 6.0)as f32).cos());


                    draw_texture_ex(self.get_texture(TextureName::Title), 149.0, 80.0, color, press_params);
                };


            },
            GameState::Win => {

                let title_params = DrawTextureParams {
                    dest_size: Some(Vec2 { x: 128.0, y: 32.0 }),
                    source: Some(Rect::new(0.0, 112.0, 128.0, 32.0)),
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                };

                let color = Color::new(1.0, 1.0, 1.0, 0.9 + 0.1 * ((get_time() * 6.0)as f32).cos());
                draw_texture_ex(self.get_texture(TextureName::Title), 149.0, 40.0, color, title_params);
                
            },
            GameState::End => {
                let title_params = DrawTextureParams {
                    dest_size: Some(Vec2 { x: 128.0, y: 32.0 }),
                    source: Some(Rect::new(0.0, 80.0, 128.0, 32.0)),
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                };

                let color = Color::new(1.0, 1.0, 1.0, 0.9 + 0.1 * ((get_time() * 6.0)as f32).cos());
                draw_texture_ex(self.get_texture(TextureName::Title), 149.0, 40.0, color, title_params);
                
            },
            _  => {}

        }
        //render start button

    }

    pub fn render_health_bar(&mut self) {
        // And the health bar decoration
        draw_texture(self.get_texture(TextureName::HealthDeco), 81.0, -48.0, WHITE);
        // Health bar

        let width = 240.0 * self.hero.get_health() as f32 / 1200.0;

        let health_params = DrawTextureParams {
            dest_size: Some(Vec2{x: width, y: 8.0}),
            source: Some(Rect::new(0.0, 0.0, width, 8.0)),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None
        };
        let color = Color::new(1.0, 1.0, 1.0, 0.9 + 0.1 * ((get_time() * 4.0)as f32).cos());
        draw_texture_ex(self.get_texture(TextureName::HealthBar), 85.0, -36.0, color, health_params);
    }

    pub fn render_letterbox_mask(&mut self) {
        // Letterbox mask (to avoid some artifact)
        draw_rectangle(0.0, -64.0, 426.0, 64.0, BLACK);
        draw_rectangle(0.0, 112.0, 426.0, 64.0, BLACK);
    }

    pub fn render_particles(&mut self) {
        // Some atmospheric particles
        for part in self.particles.iter_mut() {
            let texture = self.texture_library.get(&TextureName::ParticleOne).expect("No texture in library").clone();
            part.sprite.draw_sprite(texture, Vec2::ZERO, 1.0);
        }
    }

    pub fn render_ground_mask(&mut self) {
        // The ground to hide some lights
        let bg_params = DrawTextureParams {
            dest_size: Some(Vec2::new(426.0, 112.0)),
            source: Some(Rect::new(0.0, 0.0, 426.0, 112.0)),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None};
        draw_texture_ex(self.get_texture(TextureName::Ground), 0.0 , 0.0, WHITE, bg_params);

    }
   
    pub fn render_background(&mut self) {
        let bg_params = DrawTextureParams {
            dest_size: Some(Vec2::new(426.0, 112.0)),
            source: Some(Rect::new(0.0, 0.0, 426.0, 112.0)),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None};
        draw_texture_ex(self.get_texture(TextureName::Background), 0.0 , 0.0, WHITE, bg_params);


        // draw the light
        for light in self.lights.iter() {
            let texture = self.texture_library.get(&TextureName::Light).expect("No texture in library").clone();
            let radius = light.get_radius();
            let params = DrawTextureParams {
                dest_size: Some(Vec2 { x: 2.0 * radius, y: 2.0 * radius }),
                source: Some(Rect{x: 0.0, y: 0.0, w: 64.0, h:64.0}),
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None
            };
            draw_texture_ex(texture, light.get_position().x, light.get_position().y, light.color, params);
        }
    }

}