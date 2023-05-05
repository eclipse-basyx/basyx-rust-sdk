// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum QualifierKind {
    ConceptQualifier,
    TemplateQualifier,
    ValueQualifier,
}

impl QualifierKind {
    pub fn get(qualifier_kind: String) -> Result<QualifierKind, ()> {
        match qualifier_kind.as_str().to_lowercase().as_str() {
            "ConceptQualifier" => Ok(QualifierKind::ConceptQualifier),
            "TemplateQualifier" => Ok(QualifierKind::TemplateQualifier),
            "ValueQualifier" => Ok(QualifierKind::ValueQualifier),
            _ => Err(()),
        }
    }
}
