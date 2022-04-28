// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use crate::DataObjectTypeName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataObjectType {
    pub name: DataObjectTypeName,
}

impl DataObjectType {
    pub fn new(name: DataObjectTypeName) -> Self {
        Self { name }
    }
}
