use std::f32::consts::PI;

use winit::event::VirtualKeyCode;

use luna::audio::audio_subsystem::Sound;
use luna::gfx::texture::Sprite;
use luna::math::geo::V2;
use luna::sys::app::{Context, LunarApp, run};
use luna::world::components::SpriteComponent;
use luna::world::world::{Entity, EntityBuilder, EntityID, World};

pub struct TestApp {
    tilemap: Sprite,
    tree: Sprite,
    synth: Sound,

    world: World,
    player: Option<EntityID>,
}


impl Default for TestApp {
    fn default() -> Self {
        Self {
            tilemap: Sprite::default(),
            tree: Sprite::default(),
            synth: Sound::default(),
            world: World::new(),
            player: None,
        }
    }
}

impl LunarApp for TestApp {
    fn setup(&mut self, ctx: &mut Context) {
        let mut res = (*ctx.res).borrow_mut();

        self.tilemap = Sprite::new(
        res.load_texture("res/tilesheet.png", None).unwrap(),
        V2::new(0.5, 0.5),
        V2::new(1.0, 1.0),
        );

        self.tree = Sprite::new(
        res.load_texture("res/happy-tree.png", None).unwrap(),
        V2::new(0.5, 0.5),
        V2::new(1.0, 1.0),
        );

        self.synth = Sound::new(res.load_sound("res/synth.wav").unwrap());

        let player = Entity::builder()
            .add_sprite_component(
                SpriteComponent {
                    sprite: self.tilemap,
                    draw_pos: V2::new(0.0, 0.0),
                }
            )
            .build();

        self.player = Some(self.world.add_entity(player));
    }

    fn update(&mut self, ctx: &mut Context) {
        let r2d = &mut ctx.r2d;

        r2d.draw_sprite_ext(
            &self.tilemap,
            &V2::new(100.0, 100.0),
            &V2::new(32.0, 32.0),
            &V2::new(0.0, 0.0),
            &V2::new(32.0, 32.0),
            Some(PI / 4.0),
        );

        r2d.draw_sprite(&self.tree, &V2::new(300.0, 300.0));

        if ctx.input.key_pressed(VirtualKeyCode::Space) {
            ctx.audio.play_sound(&self.synth);
        }

        if let Some(player) = self.world.get_entity_mut(self.player.unwrap()) {
            if ctx.input.key_held(VirtualKeyCode::A) {
                player.transform.position.x -= 1.0;
            }

            if ctx.input.key_held(VirtualKeyCode::D) {
                player.transform.position.x += 1.0;
            }
        }

        self.world.update(ctx);
        self.world.render(ctx);
    }

    fn shutdown(&mut self, _ctx: &mut Context) {
    }
}

fn main() {
    let app = TestApp::default();
    pollster::block_on(run(app));
}
