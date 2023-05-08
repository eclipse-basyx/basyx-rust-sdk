// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use super::{EmbeddedDataSpecification, SubmodelElement};
use crate::LangString as LangStringNameType;
use crate::LangString as LangStringTextType;
use crate::{model_type::ModelType, qualifier::Qualifier, reference::Reference, Extension};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SubmodelElementCollection {
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<SubmodelElement>>,
}

impl SubmodelElementCollection {
    pub fn new() -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            model_type: ModelType::SubmodelElementCollection,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            value: None,
        }
    }

    pub fn add_submodel_element(&mut self, element: SubmodelElement) {
        if let Some(v) = self.value.as_mut() {
            v.push(element);
        } else {
            self.value = Some(vec![element]);
        }
    }
}

impl Default for SubmodelElementCollection {
    fn default() -> Self {
        Self::new()
    }
}
