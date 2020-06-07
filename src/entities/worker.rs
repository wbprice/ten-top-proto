use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entity, World},
    prelude::*,
};

use crate::components::{Destination, Sprite, Sprites, Velocity, Worker};

pub fn init_worker(world: &mut World, local: Transform) -> Entity {
    world
        .create_entity()
        .with(Worker {})
        .with(local)
        .with(Sprite {
            sprite: Sprites::Worker,
        })
        .with(Velocity { x: 0.0, y: 0.0 })
        .with(Destination { x: 192.0, y: 128.0 })
        .build()
}
