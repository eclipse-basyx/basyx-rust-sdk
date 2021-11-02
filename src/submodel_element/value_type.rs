use super::data_object_type::DataObjectType;
use crate::DataObjectTypeName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValueType {
    pub data_object_type: DataObjectType,
}

impl ValueType {
    pub fn new(type_name: DataObjectTypeName) -> Self {
        Self {
            data_object_type: DataObjectType { name: type_name },
        }
    }
}
