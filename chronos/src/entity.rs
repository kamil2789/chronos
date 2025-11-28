mod component_storage;
use crate::entity::{self, component_storage::ComponentStorage};

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
    pub fn new(entity_capacity: usize) -> Self {
        Self {
            next_id: 0,
            free_ids: Vec::new(),
            components: ComponentStorage::new(entity_capacity),
        }
    }

    pub fn create_entity<T: ComponentBundle>(&mut self, component_bundle: T) -> usize {
        let entidy_id = self.create_entity_id();
        component_bundle.add_to_entity(entidy_id, &mut self.components);
        entidy_id
    }

    pub fn get_component_by_id<T: 'static>(&self, entity_id: usize) -> Option<&T> {
        self.components.get_component::<T>(entity_id)
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

    use crate::components::{color::{Color, RGBA}, shape::Shape};
    use super::EntityManager;

    #[test]
    fn test_entity_manager_add_entity_with_one_component() {
        let mut entity_manager = EntityManager::new(100);

        let shape = Shape::new_triangle(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
        );

        let id = entity_manager.create_entity((shape.clone(),));
        let retrieved_shape = entity_manager.get_component_by_id::<Shape>(id).unwrap();

        assert_eq!(retrieved_shape.get_vertices(), shape.get_vertices());
        assert!(entity_manager.get_component_by_id::<Color>(id).is_none());
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
        let retrieved_shape = entity_manager.get_component_by_id::<Shape>(id).unwrap();
        let retrieved_color = entity_manager.get_component_by_id::<Color>(id).unwrap();

        assert_eq!(retrieved_shape.get_vertices(), shape.get_vertices());
        assert!(retrieved_color.is_uniform());
        assert_eq!(retrieved_color.get_uniform_color().unwrap().get(), color.get_uniform_color().unwrap().get());
    }
}