use crate::gfx::texture::Sprite;
use crate::math::geo::V2;
use crate::sys::app::Context;

pub trait Component {
    fn start(&mut self, ctx: &mut Context);
    fn update(&mut self, ctx: &mut Context);
    fn render(&mut self, ctx: &mut Context);
    fn shutdown(&mut self, ctx: &mut Context);
}

pub struct TransformComponent {
    pub position: V2,
    pub scale: V2,
    pub rotation: f32,
}

impl Default for TransformComponent {
    fn default() -> Self {
        Self {
            position: V2::new(0.0, 0.0),
            scale: V2::new(1.0, 1.0),
            rotation: 0.0,
        }
    }
}

impl Component for TransformComponent {
    fn start(&mut self, ctx: &mut Context) {
        todo!()
    }

    fn update(&mut self, ctx: &mut Context) {
        todo!()
    }

    fn render(&mut self, ctx: &mut Context) {
        todo!()
    }

    fn shutdown(&mut self, ctx: &mut Context) {
        todo!()
    }
}

pub struct HealthComponent {
    pub health: i32,
}

impl Component for HealthComponent {
    fn start(&mut self, ctx: &mut Context) {
        todo!()
    }

    fn update(&mut self, ctx: &mut Context) {
        todo!()
    }

    fn render(&mut self, ctx: &mut Context) {
        todo!()
    }

    fn shutdown(&mut self, ctx: &mut Context) {
        todo!()
    }
}

pub struct SpriteComponent {
    pub sprite: Sprite,
    pub draw_pos: V2,
}

impl Component for SpriteComponent {
    fn start(&mut self, ctx: &mut Context) {
        todo!()
    }

    fn update(&mut self, ctx: &mut Context) {
        todo!()
    }

    fn render(&mut self, ctx: &mut Context) {
        ctx.r2d.draw_sprite(&self.sprite, &self.draw_pos);
    }

    fn shutdown(&mut self, ctx: &mut Context) {
        todo!()
    }
}