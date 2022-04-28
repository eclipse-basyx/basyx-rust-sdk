// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use crate::{
    administrative_information::AdministrativeInformation,
    category::Category,
    embedded_data_specification::EmbeddedDataSpecification,
    identifier::Identifier,
    model_type::{ModelType, ModelTypeName},
    reference::Reference,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConceptDescription {
    // ConceptDescription
    pub is_case_of: Option<Reference>,

    // Identifiable
    pub identification: Identifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administration: Option<AdministrativeInformation>,

    // Referable
    pub id_short: String,
    pub model_type: ModelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,

    // HasDataSpecification
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embedded_data_specification: Vec<EmbeddedDataSpecification>,
}

impl ConceptDescription {
    pub fn new(id_short: String, identification: Identifier) -> Self {
        Self {
            identification,
            administration: None,
            id_short,
            category: None,
            model_type: ModelType::new(ModelTypeName::ConceptDescription),
            embedded_data_specification: vec![],
            is_case_of: None,
        }
    }
}
