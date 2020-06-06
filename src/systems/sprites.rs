use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{Velocity, Worker};

pub struct SpriteSystem;

impl<'s> System<'s> for SpriteSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, velocities, mut locals, time): Self::SystemData) {
        for (entity, velocity, local) in (&entities, &velocities, &mut locals).join() {
            // local.prepend_translation_x(velocity.x * time.delta_seconds());
            // local.prepend_translation_y(velocity.y * time.delta_seconds());
        }
    }
}
