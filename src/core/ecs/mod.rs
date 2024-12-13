pub mod entity;
mod resource;
pub mod world;

use std::any::{Any, TypeId};

#[derive(Debug)]
pub enum EcsError {
    ComponentNotRegistered,
}

pub struct Query {
    with_component_types: Vec<TypeId>,
}

impl Query {
    pub fn new() -> Self {
        Self {
            with_component_types: Vec::new(),
        }
    }

    // TODO: Look up the best practice way of creating inline builder-like methods like these
    pub fn with<T: Any>(mut self) -> Self {
        self.with_component_types.push(TypeId::of::<T>());
        self
    }

    pub fn build(self) -> Self {
        Self {
            with_component_types: self.with_component_types,
        }
    }
}
