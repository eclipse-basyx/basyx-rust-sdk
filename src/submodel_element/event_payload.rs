// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use crate::Reference;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EventPayload {
    pub source: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceSemanticId")]
    pub source_semantic_id: Option<Reference>,

    #[serde(rename = "observableReference")]
    pub observable_reference: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "observableSemanticId")]
    pub observable_semantic_id: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subjectId")]
    pub subject_id: Option<Reference>,

    #[serde(rename = "timeStamp")]
    pub time_stamp: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}