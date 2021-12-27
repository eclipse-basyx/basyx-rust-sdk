// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use super::{EmbeddedDataSpecification, Qualifier};
use crate::{
    category::Category,
    model_type::{ModelType, ModelTypeName},
    modeling_kind::ModelingKind,
    reference::Reference,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceElement {
    // ReferenceElement
    pub value: Option<Reference>,

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

impl ReferenceElement {
    pub fn new(id_short: String, value: Option<Reference>) -> Self {
        Self {
            id_short,
            value,
            semantic_id: None,
            embedded_data_specification: vec![],
            model_type: ModelType::new(ModelTypeName::ReferenceElement),
            kind: None,
            category: None,
            qualifiers: vec![],
        }
    }
}
