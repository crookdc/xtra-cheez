use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct ResourceRegistry {
    store: HashMap<TypeId, Box<dyn Any>>,
}

impl ResourceRegistry {
    pub fn register(&mut self, resource: impl Any) {
        self.store.insert(resource.type_id(), Box::new(resource));
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        self.store
            .get(&TypeId::of::<T>())
            .map(|boxed| boxed.downcast_ref::<T>().unwrap())
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.store
            .get_mut(&TypeId::of::<T>())
            .map(|boxed| boxed.downcast_mut::<T>().unwrap())
    }

    pub fn delete<T: Any>(&mut self) -> Option<Box<T>> {
        self.store
            .remove(&TypeId::of::<T>())
            .map(|boxed| boxed.downcast::<T>().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct WorldWidth(pub f32);

    #[test]
    fn resource_registry_get() {
        let mut registry = ResourceRegistry::default();
        let world_width = WorldWidth(100.0);
        registry.register(world_width);

        let stored = registry.get::<WorldWidth>();
        assert_eq!(stored.unwrap().0, 100.0);
    }

    #[test]
    fn resource_registry_get_mut() {
        let mut registry = ResourceRegistry::default();
        let world_width = WorldWidth(100.0);
        registry.register(world_width);
        {
            let stored: &mut WorldWidth = registry.get_mut::<WorldWidth>().unwrap();
            stored.0 += 1.0;
        }
        let stored = registry.get::<WorldWidth>();
        assert_eq!(stored.unwrap().0, 101.0);
    }

    #[test]
    fn resource_registry_delete() {
        let mut registry = ResourceRegistry::default();
        let world_width = WorldWidth(100.0);
        registry.register(world_width);
        {
            let stored = registry.get::<WorldWidth>();
            assert!(stored.is_some());
        }
        let removed = registry.delete::<WorldWidth>();
        assert!(removed.is_some());
        let stored = registry.get::<WorldWidth>();
        assert!(stored.is_none());
    }
}
