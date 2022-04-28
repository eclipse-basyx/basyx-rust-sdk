// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Category {
    VARIABLE,
    PARAMETER,
    CONSTANT,
}

impl Category {
    pub fn get(cat: String) -> Category {
        match cat.as_str() {
            "VARIABLE" => Category::VARIABLE,
            "PARAMETER" => Category::PARAMETER,
            "CONSTANT" => Category::CONSTANT,
            _ => Category::default(),
        }
    }
}
impl Default for Category {
    fn default() -> Self {
        Category::CONSTANT
    }
}
