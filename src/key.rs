// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use crate::key_types::KeyTypes;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Key {
    #[serde(rename = "type")]
    pub type_: KeyTypes,
    pub value: String,
}

impl Key {
    pub fn new(type_: KeyTypes, value: String) -> Self {
        Self {
            type_,
            value,
        }
    }
}
