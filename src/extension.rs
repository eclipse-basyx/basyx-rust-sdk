// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use crate::{DataTypeDefXsd, Reference};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Extension {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "semanticId")]
    pub semantic_id: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supplementalSemanticIds")]
    pub supplemental_semantic_ids: Option<Vec<Reference>>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valueType")]
    pub value_type: Option<DataTypeDefXsd>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "refersTo")]
    pub refers_to: Option<Vec<Reference>>,
}

impl Extension {
    pub fn new(name: String) -> Self {
        Self {
            semantic_id: None,
            supplemental_semantic_ids: None,
            name: name,
            value_type: None,
            value: None,
            refers_to: None,
        }
    }
}
