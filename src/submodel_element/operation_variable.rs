use super::SubmodelElement;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OperationVariable {
    pub submodel_element: SubmodelElement,
}

impl OperationVariable {
    pub fn new(submodel_element: SubmodelElement) -> Self {
        Self { submodel_element }
    }
}
