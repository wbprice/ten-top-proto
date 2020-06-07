use amethyst::ecs::prelude::{Entity};

use crate::components::Destination;

enum Status {
    New,
    Actionable,
    Blocked,
    InProgress,
    Completed
}

enum SubtaskType {
    MoveTo { destination: Destination },
    MoveToEntity { entity: Entity },
    SetEntityOwner { entity: Entity, owner: Entity },
    WaitForEntity { entity: Entity },
}

pub struct Subtask {
    pub action: Action,
    status: Status
}

pub struct Task {
    pub owner: Entity,
    pub status: Status,
    pub subtasks: Vec<Subtask>
}