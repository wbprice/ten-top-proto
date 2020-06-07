use amethyst::{core::transform::Transform, ecs::prelude::World, prelude::*};

use crate::components::{Sprite, Sprites, Worker};

pub fn init_worker(world: &mut World, local: Transform) {
    world
        .create_entity()
        .with(Worker {})
        .with(local)
        .with(Sprite {
            sprite: Sprites::Worker,
        })
        .build();
}
