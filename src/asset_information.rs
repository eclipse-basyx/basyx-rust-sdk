use serde::{Deserialize, Serialize};

use crate::asset_kind::AssetKind;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetInformation {
    pub asset_kind: AssetKind,
}
