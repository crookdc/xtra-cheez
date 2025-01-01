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

    pub fn get_entity_component(
        &self,
        entity: usize,
        type_id: &TypeId,
    ) -> Option<Rc<RefCell<Box<dyn Any>>>> {
        self.components
            .get(type_id)
            .map(|vec| vec.get(entity).unwrap().clone())?
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
        let mut allocated: i32 = -1;
        for (_, components) in self.components.iter_mut() {
            components.push(None);
            if allocated < 0 {
                allocated = (components.len() - 1) as i32
            }
        }
        allocated as usize
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
