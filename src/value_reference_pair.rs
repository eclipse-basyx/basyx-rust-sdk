// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use crate::Reference;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ValueReferencePair {
    pub value: String,
    #[serde(rename = "valueId")]
    pub value_id: Reference,
}
