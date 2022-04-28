// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use super::data_specification_content::DataSpecificationContent;
use crate::reference::Reference;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
