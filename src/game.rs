use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::{prelude::*, rand::gen_range};

use crate::{hero::Hero, particle::Particle};
use crate::light::Light;
use crate::ghost::Ghost;


#[derive(Eq, PartialEq, Hash)]
enum TextureName{
    Background,
    Ground,
    Ghost,
    ParticleOne,
    Hero,
    Light,
}

pub struct Game {
    texture_library: HashMap<TextureName, Texture2D>,
    particles: Vec<Particle>,
    max_monsters: i32,
    monster_timer: i32,
    monsters: Vec<Ghost>,
    lights: [Light; 6],
    hero: Hero,
}

impl Game {
    pub fn new() -> Self {

        rand::srand(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64);

        let background_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sprites/Level.png"), None);
        background_texture.set_filter(FilterMode::Nearest);
        
        let ground_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sprites/Ground.png"), None);
        ground_texture.set_filter(FilterMode::Nearest);
        
        let hero_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sprites/Hero.png"), None);
        hero_texture.set_filter(FilterMode::Nearest);
        
        let particle_one_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sprites/ParticleOne.png"), None);
        particle_one_texture.set_filter(FilterMode::Nearest);

        let light_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sprites/Light.png"), None);
        light_texture.set_filter(FilterMode::Nearest);
        
        let ghost_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sprites/MonsterOne.png"), None);
        ghost_texture.set_filter(FilterMode::Nearest);
       
        let texture_library: HashMap<TextureName, Texture2D> = HashMap::from([
            (TextureName::Background, background_texture),
            (TextureName::Ground, ground_texture),
            (TextureName::Hero, hero_texture),
            (TextureName::ParticleOne, particle_one_texture),
            (TextureName::Light, light_texture),
            (TextureName::Ghost, ghost_texture),
        ]);

        let mut particles = Vec::new();
        for _i in 0..100 {
            let part = Particle::new(gen_range(0.0, 426.0), gen_range(0.0, 100.0));
            particles.push(part);
        }

        let mut lights = [
            Light::new(118.0, 70.0, 32.0),
            Light::new(117.0, 69.0, 24.0),
            Light::new(119.0, 70.0, 30.0),
            Light::new(329.0, 70.0, 32.0),
            Light::new(328.0, 69.0, 24.0),
            Light::new(330.0, 70.0, 30.0),
        ];

        let max_monsters = 5;

        // Delay for the first birth
        let monster_timer = 5;
        // Create empty vec for monster
        let mut monsters = Vec::new();
        

        //for i in 0..5 {
        //    let m = Ghost::new(gen_range(50.0, 380.0), 52.0);
        //    monsters.push(m);
        //}

        Self {
            texture_library,
            hero: Hero::new(0.0, 0.0),
            particles,
            lights,
            max_monsters,
            monster_timer,
            monsters,
        }

    }
    pub fn update(&mut self) {
        self.monster_timer -= 1;
        if self.max_monsters > 0 && self.monster_timer == 0{
            self.monster_incubator();
            self.max_monsters -= 1;
            self.monster_timer = 20 + gen_range(30, 60);
        }
        // Clean the monster list and remove all dead monster
        self.monsters.retain(|m| m.is_active());


        self.hero.update(&mut self.monsters);

        for monster in self.monsters.iter_mut() {
            monster.update(self.hero.position);
        }

        for part in self.particles.iter_mut() {
            part.update();
        }

        for light in self.lights.iter_mut() {
            light.update();
        }
    }

    fn monster_incubator(&mut self) {
        let m = Ghost::new(gen_range(50.0, 380.0), 52.0);
        self.monsters.push(m);
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



        // The hero and thes monsters
        for monster in self.monsters.iter_mut() {
            let texture = self.texture_library.get(&TextureName::Ghost).expect("No texture in library").clone();
            monster.sprite.draw_sprite(texture, Vec2::ZERO, 1.0);
        }

        self.hero.sprite.draw_sprite(self.get_texture(TextureName::Hero), Vec2::ZERO, 1.0);




        // The ground to hide some lights
        let bg_params = DrawTextureParams {
            dest_size: Some(Vec2::new(426.0, 112.0)),
            source: Some(Rect::new(0.0, 0.0, 426.0, 112.0)),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None};
        draw_texture_ex(self.get_texture(TextureName::Ground), 0.0 , 0.0, WHITE, bg_params);

        // Some atmospheric particles
        for part in self.particles.iter_mut() {
            let texture = self.texture_library.get(&TextureName::ParticleOne).expect("No texture in library").clone();
            part.sprite.draw_sprite(texture, Vec2::ZERO, 1.0);
        }

        // Letterbox mask (to avoid some artifact)
        draw_rectangle(0.0, -64.0, 426.0, 64.0, BLACK);
        draw_rectangle(0.0, 176.0, 426.0, 64.0, BLACK);


        self.debug_info();

    }

    fn debug_info(&self) {
        // debug rendering
        let h_box = self.hero.get_collision_box(0.0, 0.0);
        draw_rectangle_lines(h_box.x , h_box.y, h_box.w , h_box.h , 1.0, RED);

        // Hero hitbox
        self.hero.debug_hitbox();

        for m in self.monsters.iter() {
            let m_box = m.get_collision_box(0.0, 0.0);
            if m.is_hitable() {

                draw_rectangle_lines(m_box.x , m_box.y, m_box.w , m_box.h , 1.0, RED);
            }
            else {
                draw_rectangle_lines(m_box.x , m_box.y, m_box.w , m_box.h , 1.0, GREEN);

            }
        }
        
        //set_default_camera();    
        //draw_text(&format!("position: {} / {}", self.hero.position.x, self.hero.position.y), 16.0, 32.0, 24.0, RED);

    } 

    fn get_texture(&self, name: TextureName) -> Texture2D {
        self.texture_library.get(&name).expect("No texture in library").clone()
    }
}