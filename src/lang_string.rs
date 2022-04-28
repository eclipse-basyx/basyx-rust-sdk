// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LangString {
    pub language: String,
    pub text: String,
}

impl LangString {
    pub fn new(language: String, text: String) -> Self {
        Self { language, text }
    }
}
