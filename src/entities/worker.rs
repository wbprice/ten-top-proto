use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::{Worker};

pub fn init_worker(world: &mut World, local: Transform) {
    world.create_entity()
        .with(Worker {})
        .with(local)
        .with()
}