// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::asset_kind::AssetKind;
use crate::{Resource, SpecificAssetId};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AssetInformation {
    #[serde(rename = "assetKind")]
    pub asset_kind: AssetKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "globalAssetId")]
    pub global_asset_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "specificAssetIds")]
    pub specific_asset_ids: Option<Vec<SpecificAssetId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetType")]
    pub asset_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultThumbnail")]
    pub default_thumbnail: Option<Resource>,
}
