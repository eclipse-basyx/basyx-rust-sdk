// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use crate::Reference;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SpecificAssetId {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "semanticId")]
    pub semantic_id: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supplementalSemanticIds")]
    pub supplemental_semantic_ids: Option<Vec<Reference>>,

    pub name: String,

    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "externalSubjectId")]
    pub external_subject_id: Option<Reference>,
}