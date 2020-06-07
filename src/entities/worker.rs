use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entity, World},
    prelude::*,
};

use crate::components::{Action, Destination, Sprite, Sprites, Status, Task, Velocity, Worker};

pub fn init_worker(world: &mut World, local: Transform) -> Entity {
    world
        .create_entity()
        .with(Worker {})
        .with(local)
        .with(Sprite {
            sprite: Sprites::Worker,
        })
        .with(Velocity { x: 0.0, y: 0.0 })
        .with(Task {
            status: Status::InProgress,
            parent: None,
            action: Action::MoveTo {
                destination: Destination { x: 192.0, y: 128.0 },
            },
        })
        .build()
}
