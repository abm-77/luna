use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use crate::audio::audio_subsystem::SoundData;
use crate::gfx::graphics_subsystem::GraphicsSubsystem;
use crate::gfx::texture;
use crate::gfx::texture::Texture;

pub type ResourceID = usize;
pub type TextureID = ResourceID;
pub type SoundID = ResourceID;

pub const WHITE_TEXTURE_ID: TextureID = 0;

pub struct ResourceManager {
    gfx: Rc<RefCell<GraphicsSubsystem>>,
    textures: Vec<Texture>,
    sounds: Vec<SoundData>,
}

impl ResourceManager {
    pub fn new (gfx: Rc<RefCell<GraphicsSubsystem>>) -> Self {
        let mut res = ResourceManager {
            gfx,
            textures: vec![],
            sounds: vec![],
        };
        res.load_texture("res/white-texture.png", Some("white-texture")).unwrap();
        return res;
    }

    pub fn load_texture (&mut self, filepath: &str, label: Option<&str>) -> Result<TextureID, io::Error> {
        let gfx = (*self.gfx).borrow();
        let img_bytes = std::fs::read(filepath).expect("Could not load image file!");

        let id= self.textures.len() as TextureID;
        let mut texture =
            texture::Texture::from_bytes(&gfx.device, &gfx.queue, img_bytes.as_slice(), label)
                .expect("Could not create texture!");
        texture.id = id;

        self.textures.push(texture);

        Ok(id)
    }

    pub fn load_sound (&mut self, filepath: &str) -> Result<SoundID, io::Error> {
        let bytes = std::fs::read(filepath).expect("Could not load audio file!");
        let mut sound_data = SoundData::from_bytes(bytes.as_slice())
            .expect("Could not create sound!");

        let id = self.sounds.len() as SoundID;
        sound_data.id = id;

        self.sounds.push(sound_data);

        Ok(id)
    }

    pub fn get_sounds(&self) -> &Vec<SoundData> {&self.sounds}
    pub fn get_sound(&self, id: SoundID) -> &SoundData {&self.sounds[id]}

    pub fn get_textures(&self) -> &Vec<Texture> {
       &self.textures
    }
    pub fn get_texture(&self, id: TextureID) -> &Texture {
        &self.textures[id]
    }

}