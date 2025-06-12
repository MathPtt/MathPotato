use std::collections::HashMap;

use uuid::Uuid;

use super::node::InfixNode;

pub mod add_node_id_and_type_to_left;
pub mod count;
pub mod does_node_exist;
pub mod get_node_by_id;
pub mod new;
pub mod put;
pub mod remove_left_and_return_its_id;
pub mod update;

#[derive(Clone, Debug)]
pub struct InfixNodeStorage {
    nodes: HashMap<Uuid, InfixNode>,
}

pub trait InfixNodeStorageApi {}
impl InfixNodeStorageApi for InfixNodeStorage {}
