// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(EnumString)]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum StateOfEvent {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
}