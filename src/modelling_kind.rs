// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]
pub enum ModellingKind {
    #[default]
    Instance,
    Template,
}

impl FromStr for ModellingKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Instance" => ModellingKind::Instance,
            "Template" => ModellingKind::Template,
            _ => return Err(()),
        })
    }
}
