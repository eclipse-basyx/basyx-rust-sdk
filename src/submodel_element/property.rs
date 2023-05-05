// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use super::EmbeddedDataSpecification;
use crate::LangString as LangStringNameType;
use crate::LangString as LangStringTextType;
use crate::{
    model_type::ModelType, qualifier::Qualifier, reference::Reference, DataTypeDefXsd, Extension,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "explorer")]
use super::ValueType;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Property {
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

    #[serde(rename = "modelType")]
    pub model_type: ModelType,

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

    // Property
    //#[cfg(feature = "explorer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    // #[cfg(not(feature = "explorer"))]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valueId")]
    pub value_id: Option<Reference>,

    // #[cfg(feature = "explorer")]
    #[serde(rename = "valueType")]
    pub value_type: DataTypeDefXsd,
    // #[cfg(not(feature = "explorer"))] // TODO clarify this feature
    // pub value_type: String,
}

impl Property {
    pub fn new(value_type: DataTypeDefXsd) -> Self {
        Self {
            // #[cfg(feature = "explorer")]
            // value: {
            //     if let Some(v) = value {
            //         v.to_string()
            //     } else {
            //         String::from("")
            //     }
            // },
            // #[cfg(not(feature = "explorer"))]
            // value,
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            model_type: ModelType::Property,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            value: None,
            value_id: None,
            //#[cfg(feature = "explorer")]
            value_type: value_type,
            // #[cfg(not(feature = "explorer"))]
            // value_type: value_type.to_string();
        }
    }
}
