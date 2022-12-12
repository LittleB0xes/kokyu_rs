
use macroquad::{audio::{Sound, play_sound, PlaySoundParams, stop_sound, load_sound_from_bytes, load_sound}, prelude::set_pc_assets_folder};

#[derive(Hash, PartialEq, Eq)]
pub enum SoundList {
    IntroSound = 0,
    Beat,
    Huh1,
    Huh2,
    Huh3,
    Death,
    Heavy,
    Sword1,
    Sword2,
}
pub struct SoundBox {
    bank: Vec<Sound>,

}

impl SoundBox {
    pub async fn new() -> Self {
        //set_pc_assets_folder("../assets");

        //let intro_sound = load_sound("amb_intro.mp3").await.unwrap();
        //let beat_sound = load_sound("heart_beat.mp3").await.unwrap();
        let intro_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/amb_intro.ogg")).await.unwrap();
        let beat_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/heart_beat.ogg")).await.unwrap();
        let huh1_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/huh_1.wav")).await.unwrap();
        let huh2_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/huh_2.wav")).await.unwrap();
        let huh3_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/huh_3.wav")).await.unwrap();
        let death_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/death.wav")).await.unwrap();
        let heavy_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/sword_heavy.wav")).await.unwrap();
        let sword1_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/sword1.wav")).await.unwrap();
        let sword2_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/sword2.wav")).await.unwrap();
        Self {
            bank: vec![
                intro_sound,
                beat_sound,
                huh1_sound,
                huh2_sound,
                huh3_sound,
                death_sound,
                heavy_sound,
                sword1_sound,
                sword2_sound,

            ]

        }


    }

    pub fn play(&self, name: SoundList) {
        let params = match name {
            SoundList::IntroSound => PlaySoundParams { looped: true, volume: 0.4, },
            SoundList::Beat => PlaySoundParams { looped: true, volume: 0.4, },
            SoundList::Huh1 => PlaySoundParams { looped: false, volume: 0.3, },
            SoundList::Huh2 => PlaySoundParams { looped: false, volume: 0.3, },
            SoundList::Huh3 => PlaySoundParams { looped: false, volume: 0.3, },
            SoundList::Death => PlaySoundParams { looped: false, volume: 0.2, },
            SoundList::Heavy => PlaySoundParams { looped: false, volume: 0.2, },
            SoundList::Sword1 => PlaySoundParams { looped: false, volume: 0.2, },
            SoundList::Sword2 => PlaySoundParams { looped: false, volume: 0.2, },
        };
        play_sound(self.bank[name as usize], params)
    }

    pub fn stop(&self, name: SoundList) {
        stop_sound(self.bank[name as usize]);
    }


}