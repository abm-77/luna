use generational_arena::{Arena, Index};

use crate::math::geo::V2;
use crate::sys::app::Context;
use crate::world::components::*;

pub type EntityID = Index;

pub struct Entity {
   pub transform: TransformComponent,
   pub sprite_component: Option<SpriteComponent>,
   pub health_component: Option<HealthComponent>,
}

impl Entity {
   pub fn builder() -> EntityBuilder {
      let mut bld = EntityBuilder::default();
      bld.transform = TransformComponent {
         position: V2::new(0.0, 0.0),
         scale: V2::new(0.0, 0.0),
         rotation: 0.0,
      };
      return bld;
   }

   pub fn update(&mut self, ctx: &mut Context) {
      if let Some(health) = &mut self.health_component { health.update(ctx); }

      if let Some(sprite) = &mut self.sprite_component {
            sprite.draw_pos = self.transform.position;
      }
   }

   pub fn render(&mut self, ctx: &mut Context) {
      if let Some(sprite) = &mut self.sprite_component { sprite.render(ctx); }
   }
}

#[derive(Default)]
pub struct EntityBuilder {
   transform: TransformComponent,
   sprite_component: Option<SpriteComponent>,
   health_component: Option<HealthComponent>,
}

impl EntityBuilder {
   pub fn add_sprite_component(mut self, sprite: SpriteComponent) -> Self {
      self.sprite_component = Some(sprite);
      self
   }

   pub fn add_health_component(mut self, health: HealthComponent) -> Self {
      self.health_component = Some(health);
      self
   }

   pub fn build (self) -> Entity {
      Entity {
         transform: self.transform,
         sprite_component: self.sprite_component,
         health_component: self.health_component,
      }
   }
}

pub struct World {
   pub entities: Arena<Entity>,
}

impl World {
   pub fn new () -> Self {
      Self {
         entities: Arena::new(),
      }
   }

   pub fn add_entity(&mut self, entity: Entity) -> EntityID {
      self.entities.insert(entity)
   }

   pub fn get_entity(&self, id: EntityID) -> Option<&Entity>  {
      self.entities.get(id)
   }

   pub fn get_entity_mut(&mut self, id: EntityID) -> Option<&mut Entity>  {
      self.entities.get_mut(id)
   }

   pub fn update(&mut self, ctx: &mut Context) {
      for (_, mut entity) in self.entities.iter_mut() {
         entity.update(ctx);
      }
   }
   pub fn render(&mut self, ctx: &mut Context) {
      for (_, mut entity) in self.entities.iter_mut() {
         entity.render(ctx);
      }
   }
}