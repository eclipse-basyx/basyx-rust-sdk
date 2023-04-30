// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use crate::ValueReferencePair;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ValueList {
    #[serde(rename = "valueReferencePairs")]
    pub value_reference_pairs: Vec<ValueReferencePair>,
}