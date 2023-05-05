// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use crate::{EmbeddedDataSpecification, Reference};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct AdministrativeInformation {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "embeddedDataSpecifications")]
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "templateId")]
    pub template_id: Option<String>,
}

impl AdministrativeInformation {
    pub fn new() -> Self {
        Self {
            embedded_data_specifications: None,
            version: None,
            revision: None,
            creator: None,
            template_id: None,
        }
    }
}
