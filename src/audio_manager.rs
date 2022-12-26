use rusty_audio::Audio;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
pub enum SoundEffect {
    Explode,
    Lose,
    Move,
    Shoot,
    Startup,
    Win
}

impl SoundEffect {
    pub fn name(&self) -> &str {
        match self {
            SoundEffect::Explode => "explode",
            SoundEffect::Lose => "lose",
            SoundEffect::Move => "move",
            SoundEffect::Shoot => "shoot",
            SoundEffect::Startup => "startup",
            SoundEffect::Win => "win"
        }
    }

    fn path(&self) -> &str {
        match self {
            SoundEffect::Explode => "sounds/kill.wav",
            SoundEffect::Lose => "sounds/lose.wav",
            SoundEffect::Move => "sounds/move.wav",
            SoundEffect::Shoot => "sounds/shoot.wav",
            SoundEffect::Startup => "sounds/startup.mp3",
            SoundEffect::Win => "sounds/win.mp3"
        }
    }
}

pub struct AudioManager {
    audio: Audio
}

impl AudioManager {
    pub fn new() -> Self {
        let mut audio = Audio::new();
        for sound in SoundEffect::iter() {
            audio.add(sound.name(), sound.path());
        }
        Self { audio }
    }

    pub fn play(&mut self, source: SoundEffect) {
        self.audio.play(source.name());
    }

    pub fn stop(&mut self) {
        self.audio.wait();
    }
}