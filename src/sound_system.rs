use std::collections::HashMap;

use macroquad::audio::{Sound, load_sound_from_bytes, play_sound_once, play_sound, PlaySoundParams, stop_sound, set_sound_volume};

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
    intro_sound: Sound,
    beat_sound: Sound,
    huh1_sound: Sound,
    huh2_sound: Sound,
    huh3_sound: Sound, 
    death_sound: Sound, 
    heavy_sound: Sound,
    sword1_sound: Sound,
    sword2_sound: Sound,
    bank: Vec<Sound>,

}

impl SoundBox {
    pub async fn new() -> Self {
    //pub async fn load_all_sounds() -> HashMap<SoundList, Sound> {
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
            intro_sound,
            beat_sound,
            huh1_sound,
            huh2_sound,
            huh3_sound,
            death_sound,
            heavy_sound,
            sword1_sound,
            sword2_sound,
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
            SoundList::IntroSound => PlaySoundParams { looped: true, volume: 0.6, },
            SoundList::Beat => PlaySoundParams { looped: true, volume: 0.6, },
            SoundList::Huh1 => PlaySoundParams { looped: false, volume: 0.4, },
            SoundList::Huh2 => PlaySoundParams { looped: false, volume: 0.4, },
            SoundList::Huh3 => PlaySoundParams { looped: false, volume: 0.4, },
            SoundList::Death => PlaySoundParams { looped: false, volume: 0.4, },
            SoundList::Heavy => PlaySoundParams { looped: false, volume: 0.3, },
            SoundList::Sword1 => PlaySoundParams { looped: false, volume: 0.3, },
            SoundList::Sword2 => PlaySoundParams { looped: false, volume: 0.3, },
        };
        play_sound(self.bank[name as usize], params)
    }

    pub fn stop(&self, name: SoundList) {
        stop_sound(self.bank[name as usize]);
    }

    pub fn set_volume(&self, name: SoundList, volume: f32) {
        set_sound_volume(self.bank[name as usize], volume)
    }

}
//pub async fn load_all_sounds() -> HashMap<SoundList,Sound> {
////pub async fn load_all_sounds() -> HashMap<SoundList, Sound> {
//    let intro_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/amb_intro.ogg")).await.unwrap();
//    let beat_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/heart_beat.ogg")).await.unwrap();
//    let huh1_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/huh_1.wav")).await.unwrap();
//    let huh2_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/huh_2.wav")).await.unwrap();
//    let huh3_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/huh_3.wav")).await.unwrap();
//    let death_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/death.wav")).await.unwrap();
//    let heavy_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/sword_heavy.wav")).await.unwrap();
//    let sword1_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/sword1.wav")).await.unwrap();
//    let sword2_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/sword2.wav")).await.unwrap();
//    let sound_bank = HashMap::from([
//        (SoundList::IntroSound, intro_sound ),
//        (SoundList::Beat, beat_sound),
//        (SoundList::Huh1, huh1_sound),
//        (SoundList::Huh2, huh2_sound),
//        (SoundList::Huh3, huh3_sound),
//        (SoundList::Death, death_sound),
//        (SoundList::Heavy, heavy_sound),
//        (SoundList::Sword1, sword1_sound),
//        (SoundList::Sword2, sword2_sound),
//        ]);
//
//    sound_bank
//    
//}