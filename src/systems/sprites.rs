use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, ReadExpect, System, Write, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{Sprite},
    resources::{SpriteResource}
};

pub struct SpriteSystem;

impl<'s> System<'s> for SpriteSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Sprite>,
        ReadExpect<'s, SpriteResource>,
        WriteStorage<'s, SpriteRender>
    );

    fn run(&mut self, (entities, sprites, sprite_resources, sprite_renders): Self::SystemData) {
        for (entity, sprite, sprite_resource, _) in (&entities, &sprites, &sprite_resources, !&sprite_renders).join() {
            // Attach the sprite according to the sprites enum.
            sprite_renders.insert(entity, SpriteRender {
                sprite_sheet: sprite_resources.clone(),
                sprite_number: 1,
            });
        }
    }
}
