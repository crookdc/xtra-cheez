use crate::core::ecs::EcsError;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

#[derive(Default)]
pub struct EntityRegistry {
    free_slots: VecDeque<usize>,
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<Box<dyn Any>>>>>>,
}

impl EntityRegistry {
    pub fn register_component<T: Any>(&mut self) {
        self.components.insert(TypeId::of::<T>(), vec![]);
    }

    pub fn get_components<T: Any>(&self) -> Option<&Vec<Option<Rc<RefCell<Box<dyn Any>>>>>> {
        self.components.get(&TypeId::of::<T>())
    }

    pub fn get_entity_component<T: Any>(&self, entity: usize) -> Option<Rc<RefCell<Box<dyn Any>>>> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|vec| vec.get(entity).unwrap().clone().unwrap())
    }

    pub fn has_entity_component(&self, entity: usize, component: &TypeId) -> bool {
        self.components
            .get(component)
            .map(|vec| vec.get(entity).unwrap().is_some())
            .unwrap_or(false)
    }

    pub fn is_registered_component(&self, type_id: TypeId) -> bool {
        self.components.contains_key(&type_id)
    }

    pub fn create_entity(&mut self) -> usize {
        // If there are freed slots in the table already they should be containing all-None cells,
        // which means we can use them immediately without any clean-up. If there are no freed cells
        // then we have no choice but to allocate a new column for the actor.
        self.free_slots
            .pop_front()
            .or_else(|| Some(self.allocate_entity_slot()))
            .unwrap()
    }

    pub fn set_entity_component(&mut self, id: usize, component: impl Any) -> Result<(), EcsError> {
        let vector = self
            .components
            .get_mut(&component.type_id())
            .ok_or(EcsError::ComponentNotRegistered)?;
        vector[id] = Some(Rc::new(RefCell::new(Box::new(component))));
        Ok(())
    }

    pub fn free_entity(&mut self, entity: usize) {
        for (_, components) in self.components.iter_mut() {
            components[entity] = None;
        }
        self.free_slots.push_back(entity);
    }

    fn allocate_entity_slot(&mut self) -> usize {
        let mut allocated = None;
        for (_, components) in self.components.iter_mut() {
            components.push(None);
            if allocated.is_none() {
                allocated = Some(components.len() - 1)
            }
        }
        allocated.unwrap()
    }
}

pub struct EntityIterator {
    length: usize,
    skipped: Vec<usize>,
    current: usize,
}

impl EntityIterator {
    pub fn new(from: &EntityRegistry) -> Self {
        let mut length = 0;
        for (_, component) in from.components.iter() {
            length = component.len();
            break;
        }
        Self {
            skipped: Vec::from(from.free_slots.clone()),
            current: 0,
            length,
        }
    }
}

impl Iterator for EntityIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.length {
            return None;
        }
        while self.skipped.contains(&self.current) {
            self.current += 1;
        }
        let res = Some(self.current);
        // We should always increment the current actor slot by at least one, the while loops
        // around this statement is to ensure that we do not proceed with an 'empty' slot as that
        // would be wasteful. Some profiling should be used  to see if it perhaps ends up being more
        // wasteful to do this double iteration.
        self.current += 1;
        while self.skipped.contains(&self.current) {
            self.current += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Health(pub f32);
    struct Speed(pub f32);

    #[test]
    fn entity_registry_register_component() {
        let mut registry = EntityRegistry::default();
        assert!(registry.get_components::<Health>().is_none());

        registry.register_component::<Health>();
        assert!(registry.get_components::<Health>().is_some());
    }

    #[test]
    fn entity_registry_with_component() {
        let mut registry = EntityRegistry::default();
        registry.register_component::<Health>();
        registry.register_component::<Speed>();
        let entity = registry.create_entity();
        registry
            .set_entity_component(entity, Health(100.0))
            .unwrap();

        // Register only health and assert that the health component exists but not the speed
        // component
        let health = &registry.get_components::<Health>().unwrap()[entity]
            .as_ref()
            .unwrap();
        assert_eq!(health.borrow().downcast_ref::<Health>().unwrap().0, 100.0);
        let speed = &registry.get_components::<Speed>().unwrap()[entity];
        assert!(speed.is_none());

        // Then register the speed component and assert that it has been added to the actor
        registry.set_entity_component(entity, Speed(15.0)).unwrap();
        let speed = &registry.get_components::<Speed>().unwrap()[entity]
            .as_ref()
            .unwrap();
        assert_eq!(speed.borrow().downcast_ref::<Speed>().unwrap().0, 15.0);
    }

    #[test]
    fn entity_iterator() {
        let mut registry = EntityRegistry::default();
        registry.register_component::<Health>();
        registry.create_entity();
        registry.create_entity();
        registry.create_entity();
        registry.create_entity();
        registry.create_entity();
        registry.create_entity();
        // Here we should have six actor slots, one for each actor. Next we simulate some of them
        // being freed.
        registry.free_slots.push_back(0);
        registry.free_slots.push_back(4);

        let mut expected = VecDeque::new();
        expected.push_front(1);
        expected.push_front(2);
        expected.push_front(3);
        expected.push_front(5);
        EntityIterator::new(&registry)
            .for_each(|entity| assert_eq!(entity, expected.pop_back().unwrap()));

        // Now we create new actor, ideally this should have taken slot index 0 as it had been
        // declared as free.
        registry.create_entity();
        let mut expected = VecDeque::new();
        expected.push_front(0);
        expected.push_front(1);
        expected.push_front(2);
        expected.push_front(3);
        expected.push_front(5);
        EntityIterator::new(&registry)
            .for_each(|entity| assert_eq!(entity, expected.pop_back().unwrap()));
    }
}
