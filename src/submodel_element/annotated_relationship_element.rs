// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use crate::embedded_data_specification::EmbeddedDataSpecification;
use crate::submodel_element::data_element_choice::DataElementChoice;
use crate::LangString as LangStringNameType;
use crate::LangString as LangStringTextType;
use crate::{Extension, ModelType, Qualifier, Reference};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AnnotatedRelationshipElement {
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

    #[serde(rename = "modelType")]
    pub model_type: ModelType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "semanticId")]
    pub semantic_id: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supplementalSemanticIds")]
    pub supplemental_semantic_ids: Option<Vec<Reference>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifiers: Option<Vec<Qualifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "embeddedDataSpecifications")]
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,

    pub first: Reference,

    pub second: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<DataElementChoice>>,
}

impl AnnotatedRelationshipElement {
    pub fn new(first: Reference, second: Reference) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            first,
            second,
            model_type: ModelType::AnnotatedRelationshipElement,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            description: None,
            embedded_data_specifications: None,
            annotations: None,
        }
    }
}
