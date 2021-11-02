use crate::DataObjectTypeName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataObjectType {
    pub name: DataObjectTypeName,
}

impl DataObjectType {
    pub fn new(name: DataObjectTypeName) -> Self {
        Self { name }
    }
}
