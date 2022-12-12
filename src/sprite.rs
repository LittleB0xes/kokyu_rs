use macroquad::prelude::*;


#[derive(Copy, Clone)]
pub struct AnimationData {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub frames: i32,
    pub speed: i32,
    pub pivot_x: i32,
    pub pivot_y: i32,
}

#[derive(Clone, Copy)]
pub struct AnimatedSprite {
    position: Vec2,
    pub source_rect: Rect,
    speed: i32,
    pub frames: i32,
    elapsed: i32,
    pub current_frame: i32,
    pub flip_x: bool,
    play: bool,
    pivot_x: i32,
    pivot_y: i32,
    color: Color,
}

impl AnimatedSprite {
    pub fn new(data: &AnimationData) -> Self {
        let source_rect = Rect::new(data.x as f32, data.y as f32, data.w as f32, data.h as f32);
        Self {
            position: Vec2::ZERO,
            source_rect,
            frames: data.frames,
            speed: data.speed,
            elapsed: 0,
            current_frame: 0,
            flip_x: false,
            play: true,
            pivot_x: data.pivot_x,
            pivot_y: data.pivot_y,
            color: WHITE
        }
    }

    fn animate(&mut self) {
        self.elapsed = (self.elapsed + 1) % self.speed;
        if self.elapsed == 0 {//self.elapsed > self.speed {
            self.current_frame = (self.current_frame + 1) % self.frames;
            self.elapsed = 0;
        }
    }

    pub fn draw_sprite(&mut self, texture: Texture2D, camera: Vec2, scale: f32) {
        if self.play {
            self.animate();
        }
        let current_source_rect = Rect {
            x: self.source_rect.x + self.source_rect.w * self.current_frame as f32,
            y: self.source_rect.y,
            w: self.source_rect.w,
            h: self.source_rect.h,
        };
        let params = DrawTextureParams {
            source: Some(current_source_rect),
            dest_size: Some(Vec2::new(self.source_rect.w * scale, self.source_rect.h * scale)),
            rotation: 0.0,
            flip_x: self.flip_x,
            flip_y: false,
            pivot: None,
        };

        draw_texture_ex(
            texture,
            (self.position.x * scale - camera.x).round(),
            (self.position.y * scale - camera.y).round(),
            self.color,
            params,
        );
    }

    pub fn set_animation(&mut self, data: &AnimationData) {
        self.source_rect = Rect::new(data.x as f32, data.y as f32, data.w as f32, data.h as f32);
        self.frames = data.frames;
        self.speed = data.speed;
        self.elapsed = 0;
    }

    pub fn set_position_to(&mut self, position: Vec2) {
        self.position = Vec2{
            x: position.x - self.pivot_x as f32,
            y: position.y - self.pivot_y as f32,
        }
    }

    pub fn set_frame(&mut self, value: i32)  {
        self.current_frame = value;
    }

    pub fn is_animation_ended(&self) -> bool{
        self.current_frame == self.frames - 1
        
    }

    pub fn play(&mut self) {
        self.play = true;
        self.current_frame = 0;
    }

    pub fn set_transparency(&mut self, value: f32) {
        self.color.a = value;
    }
}
