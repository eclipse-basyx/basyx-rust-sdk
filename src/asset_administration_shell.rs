// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use crate::{administrative_information::AdministrativeInformation, asset_information::AssetInformation,
            embedded_data_specification::EmbeddedDataSpecification, Extension,
            model_type::ModelType, reference::Reference};
use serde::{Deserialize, Serialize};
use crate::LangString as LangStringNameType;
use crate::LangString as LangStringTextType;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AssetAdministrationShell {
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

    // AssetAdministrationShell
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Reference>,

    #[serde(rename = "assetInformation")]
    pub asset_information: AssetInformation,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub submodels: Option<Vec<Reference>>,
}

impl AssetAdministrationShell {
    pub fn new(
        id: String,
        asset_information: AssetInformation
    ) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            id: id,
            embedded_data_specifications: None,
            derived_from: None,
            asset_information,
            model_type: ModelType::AssetAdministrationShell,
            description: None,
            administration: None,
            submodels: None,
        }
    }
}
