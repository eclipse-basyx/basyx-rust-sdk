// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use super::{EmbeddedDataSpecification, Qualifier};
use crate::{
    category::Category,
    model_type::{ModelType, ModelTypeName},
    modeling_kind::ModelingKind,
    reference::Reference,
    DataObjectTypeName, Value,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "explorer")]
use super::ValueType;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    // Property
    #[cfg(feature = "explorer")]
    pub value: String,
    #[cfg(not(feature = "explorer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_id: Option<Reference>,
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

impl Property {
    pub fn new(id_short: String, value: Option<Value>, value_type: DataObjectTypeName) -> Self {
        Self {
            #[cfg(feature = "explorer")]
            value: {
                if let Some(v) = value {
                    v.to_string()
                } else {
                    String::from("")
                }
            },
            #[cfg(not(feature = "explorer"))]
            value,
            value_id: None,
            semantic_id: None,
            id_short,
            category: None,
            model_type: ModelType::new(ModelTypeName::Property),
            #[cfg(feature = "explorer")]
            value_type: ValueType::new(value_type),
            #[cfg(not(feature = "explorer"))]
            value_type: value_type.to_string(),
            embedded_data_specification: vec![],
            qualifiers: vec![],
            kind: None,
        }
    }
}
