// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::key::Key;
use crate::ReferenceTypes;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Reference {
    #[serde(rename = "type")]
    pub type_: ReferenceTypes,

    /// E.g. semantic id of a standard submodel
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "referredSemanticId")]
    pub referred_semantic_id: Option<Box<Reference>>,

    /// KeyType + Value, e.g. Submodel + https://example.com/ids/123456789
    pub keys: Vec<Key>,
}

impl Reference {
    pub fn new(type_: ReferenceTypes, key: Key) -> Self {
        Self {
            type_: type_,
            referred_semantic_id: None,
            keys: vec![key],
        }
    }

    pub fn new_from_vec(type_: ReferenceTypes, keys: Vec<Key>) -> Self {
        Self {
            type_: type_,
            referred_semantic_id: None,
            keys: keys,
        }
    }
}
