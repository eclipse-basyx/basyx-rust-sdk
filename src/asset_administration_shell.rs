// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use crate::key_types::KeyTypes;
use crate::LangString as LangStringNameType;
use crate::LangString as LangStringTextType;
use crate::{
    administrative_information::AdministrativeInformation, asset_information::AssetInformation,
    embedded_data_specification::EmbeddedDataSpecification, reference::Reference, Extension, Key,
    ReferenceTypes, Submodel,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(tag = "modelType")]
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
    ///Create a new AssetAdministrationShell
    ///
    /// id should be a valid Identifier (IRI, IRDI, ...).
    ///
    /// asset_information must have at least the asset_kind.
    ///
    /// # Example:
    ///
    ///```
    ///use basyx_rs::{AssetAdministrationShell, AssetInformation, AssetKind};
    ///
    ///let my_aas = AssetAdministrationShell::new(
    ///                "https://example.com/id/123".to_string(),
    ///                AssetInformation::new(AssetKind::Instance));
    ///
    /// ```
    pub fn new(id: String, asset_information: AssetInformation) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            id,
            embedded_data_specifications: None,
            derived_from: None,
            asset_information,
            description: None,
            administration: None,
            submodels: None,
        }
    }

    pub fn add_reference_to_submodel(
        &mut self,
        submodel: &Submodel,
        type_: ReferenceTypes,
        include_referred_semantic_id: bool,
    ) {
        let mut reference =
            Reference::new(type_, Key::new(KeyTypes::Submodel, submodel.id.clone()));

        if include_referred_semantic_id {
            if let Some(rsid) = &submodel.semantic_id {
                reference.referred_semantic_id = Some(Box::new(rsid.clone()));
            }
        }

        if let Some(v) = self.submodels.as_mut() {
            v.push(reference);
        } else {
            self.submodels = Some(vec![reference]);
        }
    }

    pub fn delete_reference_to_submodel(&mut self, index: usize) {
        if let Some(v) = self.submodels.as_mut() {
            if index < v.len() {
                v.remove(index);
            }
        }
    }
}
