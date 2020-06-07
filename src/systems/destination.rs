use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Destination, Velocity};

pub struct DestinationSystem;

impl<'s> System<'s> for DestinationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Destination>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, mut velocities, destinations, locals, time): Self::SystemData,
    ) {
        let mut velocities_to_update: Vec<(Entity, Velocity)> = vec![];

        for (entity, destination, velocity, local) in
            (&entities, &destinations, &velocities, &locals).join()
        {
            let position = local.translation();

            // Ensure that velocity is moving the character in the right direction
            let mut x = 0.;
            let mut y = 0.;

            // If entity is to the left of the destination, x should be positive.
            // If entity to to the right of the destination, x should be negative.
            // If entity is lined up with the destination, x should be zero
            if position.x < destination.x {
                x = 1.;
            } else if position.x > destination.x {
                x = -1.;
            } else {
                x = 0.;
            }

            // If entity is above the destination, y should be negative.
            // If entity is below the destination, y should be positive.
            // If entity is lined up with the destination, y should be zero.
            if position.y < destination.y {
                y = 1.;
            } else if position.y > destination.y {
                y = -1.;
            } else {
                y = 0.;
            }

            velocities_to_update.push((entity, Velocity { x, y }));
        }

        for (entity, velocity) in velocities_to_update {
            velocities.insert(entity, velocity).unwrap();
        }
    }
}
