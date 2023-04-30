// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use crate::Reference;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ValueReferencePair {
    pub value: String,
    #[serde(rename = "valueId")]
    pub value_id: Reference,
}