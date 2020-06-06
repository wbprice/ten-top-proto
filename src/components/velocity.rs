use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Velocity {
    pub x: i8,
    pub y: i8,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
