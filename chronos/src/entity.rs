mod component_storage;
use crate::entity::component_storage::ComponentStorage;

pub trait ComponentBundle {
    fn add_to_entity(self, entity_id: usize, storage: &mut ComponentStorage);
}

#[derive(Default)]
pub struct EntityManager {
    next_id: usize,
    free_ids: Vec<usize>,
    components: ComponentStorage,
}

impl EntityManager {
    #[must_use]
    pub fn new(storage_capacity: usize) -> Self {
        Self {
            next_id: 0,
            free_ids: Vec::new(),
            components: ComponentStorage::new(storage_capacity),
        }
    }

    pub fn create_entity<T: ComponentBundle>(&mut self, component_bundle: T) -> usize {
        let entity_id = self.create_entity_id();
        component_bundle.add_to_entity(entity_id, &mut self.components);
        entity_id
    }

    #[must_use]
    pub fn get_component<T: 'static>(&self, entity_id: usize) -> Option<&T> {
        self.components.get_component::<T>(entity_id)
    }

    pub fn add_component<T: 'static>(&mut self, entity_id: usize, component: T) {
        self.components.add_component(entity_id, component);
    }

    #[must_use]
    pub fn has_component<T: 'static>(&self, entity_id: usize) -> bool {
        self.components.has_component::<T>(entity_id)
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        self.components.remove_all_components(entity_id);
        self.free_ids.push(entity_id);
    }

    #[must_use]
    pub fn get_entity_count(&self) -> usize {
        self.next_id - self.free_ids.len()
    }

    #[must_use]
    pub fn entity_exists(&self, entity_id: usize) -> bool {
        entity_id < self.next_id && !self.free_ids.contains(&entity_id)
    }

    fn create_entity_id(&mut self) -> usize {
        if let Some(free_id) = self.free_ids.pop() {
            free_id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            id
        }
    }
}

macro_rules! impl_component_bundle_for_tuple {
    ($($T:ident),+) => {
        impl<$($T: 'static),+> ComponentBundle for ($($T,)+) {
            fn add_to_entity(self, entity_id: usize, storage: &mut ComponentStorage) {
                #[allow(non_snake_case)]
                let ($($T,)+) = self;
                $(storage.add_component(entity_id, $T);)+
            }
        }
    };
}

impl_component_bundle_for_tuple!(T1);
impl_component_bundle_for_tuple!(T1, T2);
impl_component_bundle_for_tuple!(T1, T2, T3);
impl_component_bundle_for_tuple!(T1, T2, T3, T4);

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::EntityManager;
    use crate::components::{
        color::{Color, RGBA},
        shape::Shape,
    };

    #[test]
    fn test_entity_manager_add_entity_with_one_component() {
        let mut entity_manager = EntityManager::new(100);

        let shape = Shape::new_triangle(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
        );

        let id = entity_manager.create_entity((shape.clone(),));
        let retrieved_shape = entity_manager.get_component::<Shape>(id).unwrap();

        assert_eq!(retrieved_shape.get_vertices(), shape.get_vertices());
        assert!(entity_manager.get_component::<Color>(id).is_none());
    }

    #[test]
    fn test_entity_manager_add_entity_with_components() {
        let mut entity_manager = EntityManager::new(100);

        let shape = Shape::new_triangle(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let color = Color::Uniform(RGBA::default());

        let id = entity_manager.create_entity((shape.clone(), color.clone()));
        let retrieved_shape = entity_manager.get_component::<Shape>(id).unwrap();
        let retrieved_color = entity_manager.get_component::<Color>(id).unwrap();

        assert_eq!(retrieved_shape.get_vertices(), shape.get_vertices());
        assert!(retrieved_color.is_uniform());
        assert_eq!(
            retrieved_color.get_uniform_color().unwrap().get(),
            color.get_uniform_color().unwrap().get()
        );
    }

    #[test]
    fn test_entity_manager_remove_entity() {
        let mut entity_manager = EntityManager::new(5);
        let shape = Shape::new_triangle(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let color = Color::Uniform(RGBA::default());
        let id = entity_manager.create_entity((shape, color));

        let retrieved_shape = entity_manager.get_component::<Shape>(id).unwrap();
        assert!(retrieved_shape.get_vertices().len() == 3);
        assert_eq!(entity_manager.get_entity_count(), 1);

        entity_manager.remove_entity(id);
        assert!(entity_manager.get_component::<Shape>(id).is_none());
        assert!(entity_manager.get_component::<Color>(id).is_none());

        assert_eq!(entity_manager.get_entity_count(), 0);
    }

    #[test]
    fn test_entity_manager_add_component_to_existing_entity() {
        let mut entity_manager = EntityManager::new(10);
        let shape = Shape::new_triangle(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let id = entity_manager.create_entity((shape.clone(),));
        let color = Color::Uniform(RGBA::new(120, 130, 140, 255_f32));
        entity_manager.add_component(id, color.clone());

        let retrieved_shape = entity_manager.get_component::<Shape>(id).unwrap();
        let retrieved_color = entity_manager.get_component::<Color>(id).unwrap();

        assert_eq!(retrieved_shape, &shape);
        assert_eq!(retrieved_color, &color);
    }

    #[test]
    fn test_entity_manager_has_component() {
        let mut entity_manager = EntityManager::new(10);
        let shape = Shape::new_triangle(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let id = entity_manager.create_entity((shape.clone(),));
        assert!(entity_manager.has_component::<Shape>(id));
        assert!(!entity_manager.has_component::<Color>(id));
    }

    #[test]
    fn test_entity_manager_entity_exists() {
        let mut entity_manager = EntityManager::new(10);
        let shape = Shape::new_triangle(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let id = entity_manager.create_entity((shape.clone(),));
        assert!(entity_manager.entity_exists(id));
        assert!(!entity_manager.entity_exists(999));
    }
}
