use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModelTypeName {
    Asset,
    AssetAdministrationShell,
    BasicEvent,
    ConceptDescription,
    Event,
    File,
    Operation,
    OperationVariable,
    Property,
    Qualifier,
    Range,
    ReferenceElement,
    Submodel,
    SubmodelElementCollection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelType {
    name: ModelTypeName,
}

impl ModelType {
    pub fn new(type_name: ModelTypeName) -> Self {
        Self { name: type_name }
    }

    pub fn name(&self) -> ModelTypeName {
        self.name.clone()
    }
}
