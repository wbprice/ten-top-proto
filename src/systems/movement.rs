use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::Velocity;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, velocities, mut locals, time): Self::SystemData) {
        for (entity, velocity, local) in (&entities, &velocities, &mut locals).join() {
            local.prepend_translation_x(velocity.x * 48.0 * time.delta_seconds());
            local.prepend_translation_y(velocity.y * 48.0 * time.delta_seconds());
        }
    }
}
