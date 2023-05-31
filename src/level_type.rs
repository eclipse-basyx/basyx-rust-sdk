// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LevelType {
    pub max: bool,
    pub min: bool,
    pub nom: bool,
    pub typ: bool,
}
