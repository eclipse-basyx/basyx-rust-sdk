// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use crate::administrative_information::AdministrativeInformation;
use crate::embedded_data_specification::EmbeddedDataSpecification;
use crate::model_type::ModelType;
use crate::reference::Reference;
use serde::{Deserialize, Serialize};
use crate::Extension;
use crate::LangString as LangStringNameType;
use crate::LangString as LangStringTextType;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ConceptDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "idShort")]
    pub id_short: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayName")]
    pub display_name: Option<Vec<LangStringNameType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<LangStringTextType>>,

    #[serde(rename = "modelType")]
    pub model_type: ModelType,

    // Identifiable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administration: Option<AdministrativeInformation>,

    pub id: String,

    // HasDataSpecification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "embeddedDataSpecifications")]
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isCaseOf")]
    pub is_case_of: Option<Vec<Reference>>,
}

impl ConceptDescription {
    pub fn new(
        id: String
    ) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            id: id,
            embedded_data_specifications: None,
            model_type: ModelType::ConceptDescription,
            description: None,
            administration: None,
            is_case_of: None,
        }
    }
}
