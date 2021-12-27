// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use crate::{
    key::Key,
    model_type::{ModelType, ModelTypeName},
    reference::Reference,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Qualifier {
    // Qualifier
    pub r#type: String,

    // Constraint
    pub model_type: ModelType,

    // HasSemantics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_id: Option<Reference>,

    // ValueObject
    pub keys: Vec<Key>,
    pub value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_id: Option<Reference>,
    pub value: String,
}

impl Qualifier {
    pub fn new(
        keys: Vec<Key>,
        r#type: String,
        value_type: String,
        value_id: Option<Reference>,
        value: String,
    ) -> Self {
        Self {
            semantic_id: None,
            keys,
            r#type,
            value_type,
            value_id,
            value,
            model_type: ModelType::new(ModelTypeName::Qualifier),
        }
    }
}
