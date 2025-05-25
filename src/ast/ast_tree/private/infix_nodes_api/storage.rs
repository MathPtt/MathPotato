use std::collections::HashMap;

use uuid::Uuid;

use super::entity::InfixAstNodeInternal;

pub mod new;

#[derive(Clone, Debug)]
pub struct InfixNodeStorage {
    tree: HashMap<Uuid, InfixAstNodeInternal>,
}

pub trait InfixNodeStorageApi {}
impl InfixNodeStorageApi for InfixNodeStorage {}
