use crate::core::ecs::entity::{EntityIterator, EntityRegistry};
use crate::core::ecs::resource::ResourceRegistry;
use crate::core::ecs::{EcsError, Query};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct World {
    resource_registry: ResourceRegistry,
    entity_registry: EntityRegistry,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_resource(&mut self, resource: impl Any) {
        self.resource_registry.register(resource);
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resource_registry.get::<T>()
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resource_registry.get_mut::<T>()
    }

    pub fn delete_resource<T: Any>(&mut self) {
        self.resource_registry.delete::<T>();
    }

    pub fn register_component<T: Any>(&mut self) {
        self.entity_registry.register_component::<T>();
    }

    pub fn create_entity(&mut self) -> usize {
        self.entity_registry.create_entity()
    }

    pub fn attach_entity_component(
        &mut self,
        entity: usize,
        component: impl Any,
    ) -> Result<(), EcsError> {
        self.entity_registry.set_entity_component(entity, component)
    }

    pub fn get_entity_component<T: Any>(
        &self,
        entity: usize,
    ) -> Option<Rc<RefCell<Box<dyn Any>>>> {
        self.entity_registry.get_entity_component::<T>(entity)
    }

    pub fn resolve(&self, query: &Query) -> Vec<usize> {
        EntityIterator::new(&self.entity_registry)
            .filter(|entity| {
                let mut addable = true;
                for type_id in &query.with_component_types {
                    if !self.entity_registry.has_entity_component(*entity, &type_id) {
                        addable = false;
                    }
                }
                addable
            })
            .collect()
    }
}
