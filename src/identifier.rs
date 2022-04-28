// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use crate::key_type::KeyType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub id_type: KeyType,
    pub id: String,
}

impl Identifier {
    pub fn new(id_type: KeyType, id: String) -> Self {
        Self { id_type, id }
    }
}
