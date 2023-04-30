// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use crate::{DataTypeIec61360, LevelType, ModelType, Reference, ValueList};
use crate::LangString as LangStringDefinitionTypeIec61360;
use crate::LangString as LangStringPreferredNameTypeIec61360;
use crate::LangString as LangStringShortNameTypeIec61360;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DataSpecificationIec61360 {
    #[serde(rename = "modelType")]
    pub model_type: ModelType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dataType")]
    pub data_type: Option<DataTypeIec61360>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<Vec<LangStringDefinitionTypeIec61360>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "levelType")]
    pub level_type: Option<LevelType>,

    #[serde(rename = "preferredName")]
    pub preferred_name: Vec<LangStringPreferredNameTypeIec61360>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shortName")]
    pub short_name: Option<Vec<LangStringShortNameTypeIec61360>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceOfDefinition")]
    pub source_of_definition: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unitId")]
    pub unit_id: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valueFormat")]
    pub value_format: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valueList")]
    pub value_list: Option<ValueList>,
}