// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use crate::reference::Reference;
use serde::{Deserialize, Serialize};
use crate::DataSpecificationContent;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedDataSpecification {
    pub data_specification: Reference,
    pub data_specification_content: DataSpecificationContent,
}

impl EmbeddedDataSpecification {
    pub fn new(
        data_specification: Reference,
        data_specification_content: DataSpecificationContent,
    ) -> Self {
        Self {
            data_specification,
            data_specification_content,
        }
    }
}
