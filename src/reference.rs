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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "referredSemanticId")]
    pub referred_semantic_id: Option<Box<Reference>>,

    pub keys: Vec<Key>,
}
