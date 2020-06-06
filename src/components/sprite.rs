use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub enum Sprites {
    Worker,
}

pub struct Sprite {
    sprite: Sprites,
}

impl Component for Sprite {
    type Storage = DenseVecStorage<Self>;
}
