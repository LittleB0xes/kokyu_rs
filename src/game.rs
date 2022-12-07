use std::collections::HashMap;

use macroquad::{prelude::*, rand::gen_range};

use crate::{hero::Hero, particle::Particle};


#[derive(Eq, PartialEq, Hash)]
enum TextureName{
    Background,
    ParticleOne,
    Hero,
}


pub struct Game {
    texture_library: HashMap<TextureName, Texture2D>,
    particles: Vec<Particle>,
    hero: Hero,
}

impl Game {
    pub fn new() -> Self {
        let background_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sprites/Level.png"), None);
        background_texture.set_filter(FilterMode::Nearest);
        
        let hero_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sprites/Hero.png"), None);
        hero_texture.set_filter(FilterMode::Nearest);
        
        let particle_one_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sprites/ParticleOne.png"), None);
        particle_one_texture.set_filter(FilterMode::Nearest);
       
        let texture_library: HashMap<TextureName, Texture2D> = HashMap::from([
            (TextureName::Background, background_texture),
            (TextureName::Hero, hero_texture),
            (TextureName::ParticleOne, particle_one_texture)
        ]);

        let mut particles = Vec::new();
        for _i in 0..100 {
            let part = Particle::new(gen_range(0.0, 426.0), gen_range(0.0, 100.0));
            particles.push(part);
        }
        Self {
            texture_library,
            hero: Hero::new(0.0, 0.0),
            particles,
        }

    }
    pub fn update(&mut self) {
        self.hero.update();
        for part in self.particles.iter_mut() {
            part.update();
        }
    }

    /// An ugly experimental empiric camera setting function
    fn set_camera_view(&mut self)  {
        let ratio =  screen_width() / 1278.;
        let h = 240.0 * screen_height() / 720. / ratio;
        let camera = Camera2D::from_display_rect(Rect{x: 0.0, y: -0.5 * (h - 112.0), w: 426.0, h});
        set_camera(&camera);
    }

    pub fn render(&mut self) {
        clear_background(BLACK);

        self.set_camera_view();

        let bg_params = DrawTextureParams {
            dest_size: Some(Vec2::new(426.0, 112.0)),
            source: Some(Rect::new(0.0, 0.0, 426.0, 112.0)),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None};
        draw_texture_ex(self.get_texture(TextureName::Background), 0.0 , 0.0, WHITE, bg_params);

        self.hero.sprite.draw_sprite(self.get_texture(TextureName::Hero), Vec2::ZERO, 1.0);

        for part in self.particles.iter_mut() {
            let texture = self.texture_library.get(&TextureName::ParticleOne).expect("No texture in library").clone();
            part.sprite.draw_sprite(texture, Vec2::ZERO, 1.0);
        }


        // debug rendering
        let h_box = self.hero.get_collision_box(0.0, 0.0);
        draw_rectangle_lines(h_box.x , h_box.y, h_box.w , h_box.h , 1.0, RED);

        self.debug_info();
    }

    fn debug_info(&self) {
        set_default_camera();    
        draw_text(&format!("position: {} / {}", self.hero.position.x, self.hero.position.y), 16.0, 32.0, 24.0, RED);

    } 

    fn get_texture(&self, name: TextureName) -> Texture2D {
        self.texture_library.get(&name).expect("No texture in library").clone()
    }
}