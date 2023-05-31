// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use crate::submodel_element::{EntityType, SubmodelElement};
use crate::LangString as LangStringNameType;
use crate::LangString as LangStringTextType;
use crate::{EmbeddedDataSpecification, Extension, Qualifier, Reference, SpecificAssetId};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Entity {
    // Referable
    // HasExtension
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "idShort")]
    pub id_short: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayName")]
    pub display_name: Option<Vec<LangStringNameType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<LangStringTextType>>,

    // HasSemantics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "semanticId")]
    pub semantic_id: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supplementalSemanticIds")]
    pub supplemental_semantic_ids: Option<Vec<Reference>>,

    // Qualifiable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifiers: Option<Vec<Qualifier>>,

    // HasDataSpecification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "embeddedDataSpecifications")]
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<SubmodelElement>>,

    #[serde(rename = "entityType")]
    pub entity_type: EntityType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "globalAssetId")]
    pub global_asset_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "specificAssetIds")]
    pub specific_asset_ids: Option<Vec<SpecificAssetId>>,
}

impl Entity {
    pub fn new(entity_type: EntityType) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            statements: None,
            entity_type,
            global_asset_id: None,
            specific_asset_ids: None,
        }
    }
}
