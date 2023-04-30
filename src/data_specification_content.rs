// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use crate::ModelType;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DataSpecificationContent {
    #[serde(rename = "modelType")]
    pub model_type: ModelType,
}