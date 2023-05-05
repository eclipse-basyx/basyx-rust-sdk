// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use crate::ModelType;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DataSpecificationContent {
    #[serde(rename = "modelType")]
    pub model_type: ModelType,
}
