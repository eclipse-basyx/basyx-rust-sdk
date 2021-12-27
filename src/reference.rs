// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use serde::{Deserialize, Serialize};

use crate::key::Key;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    pub keys: Vec<Key>,
}

impl Reference {
    pub fn from_keys(keys: Vec<Key>) -> Self {
        Self { keys }
    }
}

impl Default for Reference {
    fn default() -> Self {
        Self::from_keys(vec![])
    }
}
