use std::collections::HashMap;

use uuid::Uuid;

use super::entity::I32AstEntity;

pub mod new;
#[derive(Clone, Debug)]
pub struct I32NodesApi {
    tree: HashMap<Uuid, I32AstEntity>,
}
