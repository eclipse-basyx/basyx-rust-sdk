// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ModelType {
    AnnotatedRelationshipElement,
    AssetAdministrationShell,
    BasicEventElement,
    Blob,
    Capability,
    ConceptDescription,
    DataSpecificationIec61360,
    Entity,
    File,
    MultiLanguageProperty,
    Operation,
    Property,
    Range,
    ReferenceElement,
    RelationshipElement,
    Submodel,
    SubmodelElementCollection,
    SubmodelElementList,
}

impl ModelType {
    pub fn get(key_type: String) -> Result<ModelType, ()> {
        match key_type.as_str().to_lowercase().as_str() {
            "AnnotatedRelationshipElement" => Ok(ModelType::AnnotatedRelationshipElement),
            "AssetAdministrationShell" => Ok(ModelType::AssetAdministrationShell),
            "BasicEventElement" => Ok(ModelType::BasicEventElement),
            "Blob" => Ok(ModelType::Blob),
            "Capability" => Ok(ModelType::Capability),
            "ConceptDescription" => Ok(ModelType::ConceptDescription),
            "DataSpecificationIec61360" => Ok(ModelType::DataSpecificationIec61360),
            "Entity" => Ok(ModelType::Entity),
            "File" => Ok(ModelType::File),
            "MultiLanguageProperty" => Ok(ModelType::MultiLanguageProperty),
            "Operation" => Ok(ModelType::Operation),
            "Property" => Ok(ModelType::Property),
            "Range" => Ok(ModelType::Range),
            "ReferenceElement" => Ok(ModelType::ReferenceElement),
            "RelationshipElement" => Ok(ModelType::RelationshipElement),
            "Submodel" => Ok(ModelType::Submodel),
            "SubmodelElementCollection" => Ok(ModelType::SubmodelElementCollection),
            "SubmodelElementList" => Ok(ModelType::SubmodelElementList),
            _ => Err(()),
        }
    }
}
