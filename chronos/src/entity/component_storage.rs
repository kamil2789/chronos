use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

trait Component {
    fn remove_component(&mut self, entity_id: usize);
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any(&self) -> &dyn Any;
}

struct SparseSet<T> {
    sparse: Vec<Option<usize>>,
    dense: Vec<T>,
    entities: Vec<usize>,
}

pub struct ComponentStorage {
    storages: HashMap<TypeId, Box<dyn Component>>,
    initial_capacity: usize,
}

impl<T: 'static> Component for SparseSet<T> {
    fn remove_component(&mut self, entity_id: usize) {
        self.remove(entity_id);
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ComponentStorage {
    pub fn new(initial_capacity: usize) -> Self {
        Self {
            storages: HashMap::new(),
            initial_capacity,
        }
    }

    pub fn register_component_type<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.storages.insert(
            type_id,
            Box::new(SparseSet::<T>::new(self.initial_capacity)),
        );
    }

    pub fn add_component<T: 'static>(&mut self, entity_id: usize, component: T) {
        let type_id = TypeId::of::<T>();

        if !self.storages.contains_key(&type_id) {
            self.register_component_type::<T>();
        }

        if self.needs_sparse_set_resize::<T>(entity_id) {
            self.resize_sparse_set::<T>();
        }

        if let Some(sparse_set) = self.get_mut_sparse_set::<T>() {
            sparse_set.add(entity_id, component);
        } else {
            debug_assert!(
                false,
                "Internal error: failed to get SparseSet for component type"
            );
        }
    }

    pub fn remove_all_components(&mut self, entity_id: usize) {
        for storage in self.storages.values_mut() {
            storage.remove_component(entity_id);
        }
    }

    pub fn get_component<T: 'static>(&self, entity_id: usize) -> Option<&T> {
        let sparse_set = self.get_sparse_set::<T>()?;
        sparse_set.get_component(entity_id)
    }

    pub fn has_component<T: 'static>(&self, entity_id: usize) -> bool {
        if let Some(sparse_set) = self.get_sparse_set::<T>() {
            sparse_set.get_component(entity_id).is_some()
        } else {
            false
        }
    }

    fn needs_sparse_set_resize<T: 'static>(&self, entity_id: usize) -> bool {
        if let Some(sparse_set) = self.get_sparse_set::<T>() {
            entity_id >= sparse_set.get_sparse_array_size()
        } else {
            false
        }
    }

    fn resize_sparse_set<T: 'static>(&mut self) {
        self.initial_capacity *= 2;
        let initial_capacity = self.initial_capacity;
        if let Some(sparse_set) = self.get_mut_sparse_set::<T>() {
            sparse_set.resize(initial_capacity);
        }
    }

    fn get_mut_sparse_set<T: 'static>(&mut self) -> Option<&mut SparseSet<T>> {
        let type_id = TypeId::of::<T>();
        let storage = self.storages.get_mut(&type_id)?;
        storage.as_any_mut().downcast_mut::<SparseSet<T>>()
    }

    fn get_sparse_set<T: 'static>(&self) -> Option<&SparseSet<T>> {
        let type_id = TypeId::of::<T>();
        let storage = self.storages.get(&type_id)?;
        storage.as_any().downcast_ref::<SparseSet<T>>()
    }
}

impl Default for ComponentStorage {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl<T> SparseSet<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            sparse: vec![None; capacity],
            dense: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        self.sparse.resize(new_size, None);
    }

    pub fn remove(&mut self, entity_id: usize) {
        if let Some(index) = self.get_component_dense_index(entity_id) {
            self.dense.swap_remove(index);

            let last_entity_id = *self
                .entities
                .last()
                .expect("This should not fail, internal logic error");
            self.entities.swap_remove(index);

            if index < self.entities.len() {
                self.sparse[last_entity_id] = Some(index);
            }
            self.sparse[entity_id] = None;
        }
    }

    pub fn add(&mut self, entity_id: usize, component: T) {
        if entity_id < self.sparse.len() {
            self.dense.push(component);
            self.entities.push(entity_id);
            let index = self.dense.len() - 1;
            self.sparse[entity_id] = Some(index);
        } else {
            debug_assert!(
                entity_id < self.sparse.len(),
                "ComponentStorage should ensure entity_id is valid"
            );
        }
    }

    pub fn get_component(&self, entity_id: usize) -> Option<&T> {
        if entity_id < self.sparse.len()
            && let Some(entity_index) = self.sparse[entity_id] {
                return self.dense.get(entity_index);
            }
        None
    }

    pub fn get_sparse_array_size(&self) -> usize {
        self.sparse.len()
    }

    fn get_component_dense_index(&self, entity_id: usize) -> Option<usize> {
        if let Some(dense_index_opt) = self.sparse.get(entity_id)
            && let Some(dense_index) = dense_index_opt {
                return Some(*dense_index);
            }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentStorage;
    use super::SparseSet;

    #[test]
    fn test_sparse_set_new() {
        let sparse_set: SparseSet<String> = SparseSet::new(0);
        assert!(sparse_set.dense.is_empty());
        assert!(sparse_set.sparse.is_empty());
        assert!(sparse_set.entities.is_empty());
    }

    #[test]
    fn test_sparse_set_add_component() {
        let mut sparse_set: SparseSet<String> = SparseSet::new(10);
        sparse_set.add(0, "Hello".to_string());
        sparse_set.add(1, "World".to_string());
        sparse_set.add(2, "Engine".to_string());

        assert_eq!(sparse_set.dense.len(), 3);
        assert_eq!(sparse_set.entities.len(), 3);
        assert_eq!(sparse_set.get_component(0).unwrap(), "Hello");
        assert_eq!(sparse_set.get_component(1).unwrap(), "World");
        assert_eq!(sparse_set.get_component(2).unwrap(), "Engine");
    }

    #[test]
    fn test_sparse_set_remove_component() {
        let mut sparse_set: SparseSet<String> = SparseSet::new(10);
        sparse_set.add(0, "Hello".to_string());
        sparse_set.add(5, "World".to_string());
        sparse_set.add(7, "Engine".to_string());

        sparse_set.remove(5);

        assert_eq!(sparse_set.dense.len(), 2);
        assert_eq!(sparse_set.entities.len(), 2);

        assert!(sparse_set.get_component(5).is_none());
        assert!(sparse_set.get_component(1).is_none());

        assert_eq!(sparse_set.get_component(0).unwrap(), "Hello");
        assert_eq!(sparse_set.get_component(7).unwrap(), "Engine");
    }

    #[test]
    fn test_sparse_set_add_remove_mixed() {
        let mut sparse_set: SparseSet<String> = SparseSet::new(10);
        sparse_set.add(0, "Hello".to_string());
        sparse_set.add(5, "World".to_string());
        sparse_set.add(7, "Engine".to_string());

        sparse_set.remove(5);
        sparse_set.add(2, "Chronos".to_string());

        assert_eq!(sparse_set.dense.len(), 3);
        assert_eq!(sparse_set.entities.len(), 3);

        assert!(sparse_set.get_component(5).is_none());
        assert!(sparse_set.get_component(1).is_none());

        assert_eq!(sparse_set.get_component(0).unwrap(), "Hello");
        assert_eq!(sparse_set.get_component(7).unwrap(), "Engine");
        assert_eq!(sparse_set.get_component(2).unwrap(), "Chronos");
    }

    #[test]
    fn test_sparse_set_resize() {
        let mut sparse_set: SparseSet<String> = SparseSet::new(2);
        sparse_set.resize(5);
        assert_eq!(sparse_set.get_sparse_array_size(), 5);

        sparse_set.add(3, "Resize".to_string());
        assert_eq!(sparse_set.get_component(3).unwrap(), "Resize");
    }

    #[test]
    fn test_component_storage_resize() {
        let mut storage = ComponentStorage::new(2);
        storage.add_component(0, "First".to_string());
        storage.add_component(1, "Second".to_string());

        assert_eq!(storage.initial_capacity, 2);

        storage.add_component(3, "Third".to_string());
        assert_eq!(storage.initial_capacity, 4);
    }

    #[test]
    fn test_component_storage_has_component() {
        let mut storage = ComponentStorage::new(2);
        storage.add_component(0, "First".to_string());

        assert!(storage.has_component::<String>(0));
        assert!(!storage.has_component::<String>(1));
    }
}
