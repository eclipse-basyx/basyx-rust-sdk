// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use crate::{DataSpecificationIec61360, Reference};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EmbeddedDataSpecification {
    #[serde(rename = "dataSpecification")]
    pub data_specification: Reference,
    #[serde(rename = "dataSpecificationContent")]
    pub data_specification_content: DataSpecificationIec61360,
}