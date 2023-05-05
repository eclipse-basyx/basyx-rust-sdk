// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: MIT

use crate::asset_administration_shell::AssetAdministrationShell;
use crate::concept_description::ConceptDescription;
use crate::submodel::Submodel;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Environment {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetAdministrationShells")]
    pub asset_administration_shells: Option<Vec<AssetAdministrationShell>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub submodels: Option<Vec<Submodel>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conceptDescriptions")]
    pub concept_descriptions: Option<Vec<ConceptDescription>>,
}
