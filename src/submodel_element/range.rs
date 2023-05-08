// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use super::EmbeddedDataSpecification;
use crate::{DataTypeDefXsd, LangString as LangStringNameType};
use crate::LangString as LangStringTextType;
use crate::{model_type::ModelType, qualifier::Qualifier, reference::Reference, Extension};
use serde::{Deserialize, Serialize};

// #[cfg(feature = "explorer")]
// use super::ValueType;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Range {
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

    // Range
    #[cfg(feature = "explorer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,

    // #[cfg(not(feature = "explorer"))]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub min: Option<Value>,
    #[cfg(feature = "explorer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    // #[cfg(not(feature = "explorer"))]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub max: Option<Value>,
    //#[cfg(feature = "explorer")]
    pub value_type: DataTypeDefXsd,
    //#[cfg(not(feature = "explorer"))]
    //pub value_type: String,
}

// TODO implement after clarification
// impl Range {
//     pub fn new(
//         id_short: String,
//         min: Option<Value>,
//         max: Option<Value>,
//         value_type: DataObjectTypeName,
//     ) -> Self {
//         Self {
//             #[cfg(feature = "explorer")]
//             min: {
//                 if let Some(v) = min {
//                     v.to_string()
//                 } else {
//                     String::from("")
//                 }
//             },
//             #[cfg(not(feature = "explorer"))]
//             min,
//             #[cfg(feature = "explorer")]
//             max: {
//                 if let Some(v) = max {
//                     v.to_string()
//                 } else {
//                     String::from("")
//                 }
//             },
//             #[cfg(not(feature = "explorer"))]
//             max,
//             semantic_id: None,
//             embedded_data_specification: vec![],
//             id_short,
//             category: None,
//             model_type: ModelType::Range,
//             #[cfg(feature = "explorer")]
//             value_type: ValueType::new(value_type),
//             #[cfg(not(feature = "explorer"))]
//             value_type: value_type.to_string(),
//             kind: None,
//             qualifiers: vec![],
//         }
//     }
// }
