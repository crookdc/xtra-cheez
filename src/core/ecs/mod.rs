pub mod component;
pub mod entity;
mod resource;

use crate::core::ecs::entity::{EntityIterator, EntityRegistry};
use crate::core::ecs::resource::ResourceRegistry;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::rc::Rc;

pub type EntityID = usize;

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

pub struct ECSBuilder {
    subject: ECS,
}

impl ECSBuilder {
    pub fn new() -> Self {
        Self {
            subject: ECS {
                entity_registry: EntityRegistry::default(),
                resource_registry: ResourceRegistry::default(),
            },
        }
    }

    pub fn with_component<T: Any>(mut self) -> Self {
        self.subject.register_component::<T>();
        self
    }

    pub fn with_resource(mut self, resource: impl Any) -> Self {
        self.subject.register_resource(resource);
        self
    }

    pub fn build(self) -> ECS {
        self.subject
    }
}

pub trait System {
    fn tick(&self, ecs: &mut ECS, delta_time: f32);
}

#[derive(Default)]
pub struct ECS {
    entity_registry: EntityRegistry,
    resource_registry: ResourceRegistry,
}

impl ECS {
    pub fn register_component<T: Any>(&mut self) {
        self.entity_registry.register_component::<T>();
    }

    pub fn register_resource(&mut self, resource: impl Any) {
        self.resource_registry.register(resource)
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resource_registry.get::<T>()
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resource_registry.get_mut::<T>()
    }

    pub fn create_entity(&mut self) -> EntityID {
        self.entity_registry.create_entity()
    }

    pub fn attach_component(&mut self, id: EntityID, component: impl Any) -> Result<(), EcsError> {
        self.entity_registry.set_entity_component(id, component)
    }

    pub fn get_component<T: Any>(&self, entity: EntityID) -> Option<Rc<RefCell<Box<dyn Any>>>> {
        self.entity_registry
            .get_entity_component(entity, &TypeId::of::<T>())
    }

    pub fn update_component<'a, T: Any + Clone>(
        &mut self,
        entity: EntityID,
        func: &'a mut dyn FnMut(T) -> T,
    ) -> Result<(), EcsError> {
        let cloned = self.clone_component::<T>(entity);
        if let Some(clone) = cloned {
            let updated = func(clone);
            self.entity_registry.set_entity_component(entity, updated)?
        }
        Ok(())
    }

    pub fn clone_component<T: Any + Clone>(&self, entity: EntityID) -> Option<T> {
        self.get_component::<T>(entity)
            .map(|component| component.borrow_mut().downcast_mut::<T>().unwrap().clone())
    }

    pub fn query(&self, query: &Query) -> Vec<EntityID> {
        EntityIterator::new(&self.entity_registry)
            .filter(|entity| {
                let mut matched = true;
                for type_id in &query.with_component_types {
                    if self
                        .entity_registry
                        .get_entity_component(*entity, type_id)
                        .is_none()
                    {
                        matched = false;
                    }
                }
                matched
            })
            .collect()
    }
}

pub struct EntitySpecification {
    components: Vec<Box<dyn Any>>,
}

impl EntitySpecification {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn with(mut self, component: impl Any) -> Self {
        self.components.push(Box::new(component));
        self
    }
}
