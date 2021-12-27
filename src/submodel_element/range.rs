// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use super::{EmbeddedDataSpecification, Qualifier};
use crate::{
    category::Category, model_type::ModelType, model_type::ModelTypeName,
    modeling_kind::ModelingKind, reference::Reference, DataObjectTypeName, Value,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "explorer")]
use super::ValueType;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    // Range
    #[cfg(feature = "explorer")]
    pub min: String,
    #[cfg(not(feature = "explorer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Value>,
    #[cfg(feature = "explorer")]
    pub max: String,
    #[cfg(not(feature = "explorer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Value>,
    #[cfg(feature = "explorer")]
    pub value_type: ValueType,
    #[cfg(not(feature = "explorer"))]
    pub value_type: String,

    // SubmodelElement
    pub id_short: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ModelingKind>,

    // Referable
    pub model_type: ModelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,

    // HasDataSpecification
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embedded_data_specification: Vec<EmbeddedDataSpecification>,

    // HasSemantics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_id: Option<Reference>,

    // Qualifiable
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub qualifiers: Vec<Qualifier>,
}

impl Range {
    pub fn new(
        id_short: String,
        min: Option<Value>,
        max: Option<Value>,
        value_type: DataObjectTypeName,
    ) -> Self {
        Self {
            #[cfg(feature = "explorer")]
            min: {
                if let Some(v) = min {
                    v.to_string()
                } else {
                    String::from("")
                }
            },
            #[cfg(not(feature = "explorer"))]
            min,
            #[cfg(feature = "explorer")]
            max: {
                if let Some(v) = max {
                    v.to_string()
                } else {
                    String::from("")
                }
            },
            #[cfg(not(feature = "explorer"))]
            max,
            semantic_id: None,
            embedded_data_specification: vec![],
            id_short,
            category: None,
            model_type: ModelType::new(ModelTypeName::Range),
            #[cfg(feature = "explorer")]
            value_type: ValueType::new(value_type),
            #[cfg(not(feature = "explorer"))]
            value_type: value_type.to_string(),
            kind: None,
            qualifiers: vec![],
        }
    }
}
