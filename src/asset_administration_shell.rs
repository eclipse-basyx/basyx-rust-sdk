// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use crate::{
    administrative_information::AdministrativeInformation, asset_information::AssetInformation,
    category::Category, embedded_data_specification::EmbeddedDataSpecification,
    identifier::Identifier, model_type::ModelType, model_type::ModelTypeName, reference::Reference,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetAdministrationShell {
    // AssetAdministrationShell
    pub asset_information: AssetInformation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derived_from: Option<Reference>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub submodels: Vec<Reference>,

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

impl AssetAdministrationShell {
    pub fn new(
        id_short: String,
        asset_information: AssetInformation,
        identification: Identifier,
    ) -> Self {
        Self {
            id_short,
            asset_information,
            identification,
            derived_from: None,
            submodels: vec![],
            administration: None,
            model_type: ModelType::new(ModelTypeName::AssetAdministrationShell),
            category: None,
            embedded_data_specification: vec![],
        }
    }
}
