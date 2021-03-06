use amethyst::{
    ecs::prelude::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{Sprite, Sprites},
    resources::SpriteResource,
};

pub struct SpriteSystem;

impl<'s> System<'s> for SpriteSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Sprite>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, SpriteResource>,
    );

    fn run(&mut self, (entities, sprites, mut sprite_renders, sprite_resource): Self::SystemData) {
        let mut sprites_to_insert: Vec<(Entity, SpriteRender)> = vec![];
        for (entity, sprite, _) in (&entities, &sprites, !&sprite_renders).join() {
            let sprite_sheet = sprite_resource.sprite_sheet.clone();
            // Attach the sprite according to the sprites enum.
            match sprite.sprite {
                Sprites::Worker => {
                    sprites_to_insert.push((
                        entity,
                        SpriteRender {
                            sprite_sheet: sprite_sheet,
                            sprite_number: 0,
                        },
                    ));
                }
            }
        }

        for (entity, sprite_render) in sprites_to_insert {
            sprite_renders.insert(entity, sprite_render).unwrap();
        }
    }
}
