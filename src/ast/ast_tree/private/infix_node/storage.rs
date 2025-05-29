use std::collections::HashMap;

use uuid::Uuid;

use super::node::InfixNode;

pub mod add_node_id_and_type_to_left;
pub mod does_node_exist;
pub mod get_node_by_id;
pub mod update;

#[derive(Clone, Debug)]
pub struct InfixNodeStorage {
    tree: HashMap<Uuid, InfixNode>,
}

pub trait InfixNodeStorageApi {}
impl InfixNodeStorageApi for InfixNodeStorage {}
