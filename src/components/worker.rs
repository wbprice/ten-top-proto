use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Worker;

impl Component for Worker {
    type Storage = DenseVecStorage<Self>;
}
