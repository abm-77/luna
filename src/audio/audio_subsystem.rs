use std::cell::RefCell;
use std::rc::Rc;

use soloud::*;

use crate::sys::resource_manager::{ResourceManager, SoundID};

pub struct Sound {
    pub id: SoundID,
}

pub struct SoundData {
    pub id: SoundID,
    pub wav: Wav,
}

impl Default for Sound {
    fn default() -> Self {
        Self {
            id: 0,
        }
    }
}

impl Sound {
    pub fn new (sound_id: SoundID) -> Self {
        Self {
            id: sound_id,
        }
    }
}

impl SoundData {
    pub fn from_bytes (bytes: &[u8]) -> Result<Self, SoloudError> {
        let mut wav = audio::Wav::default();
        wav.load_mem(bytes).unwrap();

        Ok(Self{
            id: 0,
            wav,
        })
    }
}

pub struct AudioSubsystem {
    res: Rc<RefCell<ResourceManager>>,
    sound: Soloud,
}

impl AudioSubsystem {
    pub fn new (res: Rc<RefCell<ResourceManager>>) -> Self {
        Self {
            res,
            sound: Soloud::default().unwrap(),
        }
    }

    pub fn play_sound(&self, sound: &Sound) -> Handle {
        let res = (*self.res).borrow();
        let sound_data = res.get_sound(sound.id);
        self.sound.play(&sound_data.wav)
    }
}