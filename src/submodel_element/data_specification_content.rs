// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use crate::{reference::Reference, LangString};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataSpecificationContent {
    pub preferred_name: Vec<LangString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub definition: Vec<LangString>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub short_name: Vec<LangString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_of_definition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<Reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_format: Option<String>,
}

impl DataSpecificationContent {
    pub fn new(preferred_name: Vec<LangString>) -> Self {
        Self {
            preferred_name,
            short_name: vec![],
            unit: None,
            unit_id: None,
            value_format: None,
            source_of_definition: None,
            symbol: None,
            data_type: None,
            definition: vec![],
        }
    }
}
