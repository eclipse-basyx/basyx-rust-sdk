// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use crate::concept_description::ConceptDescription;
use crate::submodel::Submodel;
use crate::{asset::Asset, asset_administration_shell::AssetAdministrationShell};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub asset_administration_shells: Vec<AssetAdministrationShell>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub submodels: Vec<Submodel>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<Asset>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub concept_descriptions: Vec<ConceptDescription>,
}
