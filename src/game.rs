use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::{prelude::*, rand::gen_range};

use crate::sound_system::{SoundList, SoundBox};
use crate::{hero::Hero, particle::Particle};
use crate::light::Light;
use crate::ghost::Ghost;

mod rendering;


#[derive(Eq, PartialEq, Hash)]
enum TextureName{
    Background,
    HealthDeco,
    HealthBar,
    Ground,
    Ghost,
    ParticleOne,
    Hero,
    Light,
    Title,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum TransitionName {
    FadeIn,
    FadeOut
}

#[derive(Copy, Clone)]
pub enum GameState {
    Intro,
    Game,
    Win,
    End
}

pub struct Game {
    state: GameState,
    texture_library: HashMap<TextureName, Texture2D>,
    particles: Vec<Particle>,
    max_monsters: i32,
    monster_timer: i32,
    monsters: Vec<Ghost>,
    colliders: Vec<Rect>,
    lights: [Light; 6],
    hero: Hero,

    transition_alpha: f32,
    transition: TransitionName,
    transition_finished: bool,

    ambiance_on: bool,
    sound_bank: SoundBox

}

impl Game {
    pub fn new(sound_bank: SoundBox) -> Self {

        //rand::srand(SystemTime::now()
        //    .duration_since(UNIX_EPOCH)
        //    .unwrap()
        //    .as_millis() as u64);

        let state = GameState::Intro;
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
       
        let health_container_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sprites/Health_deco.png"), None);
        health_container_texture.set_filter(FilterMode::Nearest);
        
        let health_bar_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sprites/Health_bar.png"), None);
        health_bar_texture.set_filter(FilterMode::Nearest);
        
        let title_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sprites/Title.png"), None);
        title_texture.set_filter(FilterMode::Nearest);
       
        let texture_library: HashMap<TextureName, Texture2D> = HashMap::from([
            (TextureName::Background, background_texture),
            (TextureName::Ground, ground_texture),
            (TextureName::Hero, hero_texture),
            (TextureName::ParticleOne, particle_one_texture),
            (TextureName::Light, light_texture),
            (TextureName::Ghost, ghost_texture),
            (TextureName::HealthDeco, health_container_texture),
            (TextureName::HealthBar, health_bar_texture),
            (TextureName::Title, title_texture),
        ]);

        let mut particles = Vec::new();
        for _i in 0..100 {
            let part = Particle::new(gen_range(0.0, 426.0), gen_range(0.0, 100.0));
            particles.push(part);
        }

        let lights = [
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
        let monsters = Vec::new();


        // Level collider (Ground, left and right wall)
        let colliders = vec![
            Rect{x: 0.0, y: 101.0, w: 426.0, h: 16.0},      // Ground
            Rect{x: -16.0, y: 0.0, w: 16.0, h: 112.0},      // Left border
            Rect{x: 426.0, y: 0.0, w: 16.0, h: 112.0},      // Right border

        ];

        // Sound Loading


        Self {
            state,
            texture_library,
            hero: Hero::new(0.0, 0.0, 20),
            particles,
            lights,
            max_monsters,
            colliders,
            monster_timer,
            monsters,

            transition_alpha: 1.0,
            transition: TransitionName::FadeIn,
            transition_finished: false,

            ambiance_on: false,

            sound_bank,

        }

    }
    
    pub fn update(&mut self) {
        match self.state {
            GameState::Intro => {
                if !self.ambiance_on {
                    self.ambiance_on = true;
                    self.sound_bank.play(SoundList::IntroSound);
                }
                self.update_decoration();
                if self.transition_finished && self.transition == TransitionName::FadeOut{
                    self.state = GameState::Game;
                    self.transition = TransitionName::FadeIn;
                    self.ambiance_on = false;
                    self.sound_bank.stop(SoundList::IntroSound);
                }
                if is_key_pressed(KeyCode::Space) {
                    self.transition = TransitionName::FadeOut;
                }


               
            },
            GameState::Game => {
                if !self.ambiance_on {
                    self.ambiance_on = true;
                    self.sound_bank.play(SoundList::IntroSound);
                    self.sound_bank.play(SoundList::Beat);
                }

                self.monster_timer -= 1;
                if self.max_monsters > 0 && self.monster_timer == 0{
                    self.monster_incubator();
                    self.max_monsters -= 1;
                    self.monster_timer = 20 + gen_range(30, 60);
                }
                // Clean the monster list and remove all dead monster
                self.monsters.retain(|m| m.is_active());


                self.hero.update(&mut self.monsters, &self.colliders, &self.sound_bank);

                for monster in self.monsters.iter_mut() {
                    monster.update(self.hero.position);
                }

                self.update_decoration();

                // End of game
                if self.hero.is_dead() {
                    self.state = GameState::End;
                    self.sound_bank.stop(SoundList::Beat);
                }
                else if self.max_monsters == 0 && self.monsters.len() == 0{
                    self.state = GameState::Win;

                }

            },
            GameState::End => {
                self.update_decoration();
                if is_key_pressed(KeyCode::Space) {
                    self.reset_game();
                    self.state = GameState::Game;

                    self.sound_bank.stop(SoundList::IntroSound);
                    self.ambiance_on = false;
                }
            },
            GameState::Win => {

                self.update_decoration();
                if is_key_pressed(KeyCode::Space) {
                    self.reset_game();
                    self.state = GameState::Game;

                    self.sound_bank.stop(SoundList::IntroSound);
                    self.sound_bank.stop(SoundList::Beat);
                    self.ambiance_on = false;
                }
            }
        }


        // Transition screen update
        match self.transition {
            TransitionName::FadeIn => {
                self.transition_finished = false;
                self.transition_alpha -= 0.01;
                if self.transition_alpha < 0.0 {
                    self.transition_alpha = 0.0;
                    self.transition_finished = true;
                }


            }
            TransitionName::FadeOut => {
                self.transition_finished = false;
                self.transition_alpha += 0.01;
                if self.transition_alpha > 1.0 {
                    self.transition_finished = true;
                    self.transition_alpha = 1.0;
                }
            }
        }
    }

    fn update_decoration(&mut self) {
        for part in self.particles.iter_mut() {
            part.update();
        }

        for light in self.lights.iter_mut() {
            light.update();
        }
    }


    pub fn render(&mut self) {
        clear_background(BLACK);
        self.set_camera_view();

        self.render_background();

        match self.state {
            GameState::Intro => {
                self.render_title_screen(self.state);
            },
            GameState::Game => {
                // The hero and thes monsters
                for monster in self.monsters.iter_mut() {
                    let texture = self.texture_library.get(&TextureName::Ghost).expect("No texture in library").clone();
                    monster.sprite.draw_sprite(texture, Vec2::ZERO, 1.0);
                }

                self.hero.sprite.draw_sprite(self.get_texture(TextureName::Hero), Vec2::ZERO, 1.0);

                self.render_ground_mask();
                self.render_particles();
                self.render_letterbox_mask();
                self.render_health_bar();
            },
            GameState::End => {

                self.render_title_screen(GameState::End);
                self.hero.sprite.draw_sprite(self.get_texture(TextureName::Hero), Vec2::ZERO, 1.0);
            },
            GameState::Win => {
                self.render_title_screen(GameState::Win);
                self.hero.sprite.draw_sprite(self.get_texture(TextureName::Hero), Vec2::ZERO, 1.0);

            },
        }

        // Transition screen
        let color = Color { r: 0.0, g: 0.0, b: 0.0, a: self.transition_alpha };
        draw_rectangle(0.0, -64.0, 426.0, 240.0, color);

        //self.debug_info();

    }



    fn get_texture(&self, name: TextureName) -> Texture2D {
        self.texture_library.get(&name).expect("No texture in library").clone()
    }
    
    fn reset_game(&mut self) {
        self.max_monsters = 5;
        self.monsters = Vec::new();
        self.monster_timer = 5;
        self.hero = Hero::new(0.0, 0.0, 20);
        self.state = GameState::Intro;
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
 
    //fn debug_info(&mut self) {
    //    // Reset game
    //    if is_key_pressed(KeyCode::Tab) {self.reset_game()}

    //    // debug rendering
    //    let h_box = self.hero.get_collision_box(0.0, 0.0);
    //    draw_rectangle_lines(h_box.x , h_box.y, h_box.w , h_box.h , 1.0, RED);

    //    // Hero hitbox
    //    self.hero.debug_hitbox();

    //    for m in self.monsters.iter() {
    //        let m_box = m.get_collision_box(0.0, 0.0);
    //        if m.is_hitable() {

    //            draw_rectangle_lines(m_box.x , m_box.y, m_box.w , m_box.h , 1.0, RED);
    //        }
    //        else {
    //            draw_rectangle_lines(m_box.x , m_box.y, m_box.w , m_box.h , 1.0, GREEN);

    //        }
    //    }
    //    
    //    //set_default_camera();    
    //    //draw_text(&format!("position: {} / {}", self.hero.position.x, self.hero.position.y), 16.0, 32.0, 24.0, RED);

    //} 
}