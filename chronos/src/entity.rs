mod component_storage;
use crate::entity::component_storage::ComponentStorage;

pub struct EntityManager {
    next_id: usize,
    free_ids: Vec<usize>,
    components: ComponentStorage,
}
