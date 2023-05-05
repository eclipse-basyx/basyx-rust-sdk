// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use super::{EmbeddedDataSpecification, SubmodelElement};
use crate::submodel_element::AasSubmodelElements;
use crate::LangString as LangStringNameType;
use crate::LangString as LangStringTextType;
use crate::{
    model_type::ModelType, qualifier::Qualifier, reference::Reference, DataTypeDefXsd, Extension,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SubmodelElementList {
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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "semanticId")]
    pub semantic_id: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supplementalSemanticIds")]
    pub supplemental_semantic_ids: Option<Vec<Reference>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifiers: Option<Vec<Qualifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "embeddedDataSpecifications")]
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,

    // SubmodelElementList
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "orderRelevant")]
    pub order_relevant: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "semanticIdListElement")]
    pub semantic_id_list_element: Option<Reference>,

    #[serde(rename = "typeValueListElement")]
    pub type_value_list_element: AasSubmodelElements,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valueTypeListElement")]
    pub value_type_list_element: Option<DataTypeDefXsd>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<SubmodelElement>>,
}

impl SubmodelElementList {
    pub fn new(type_value_list_element: AasSubmodelElements) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            model_type: ModelType::SubmodelElementList,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            order_relevant: None,
            semantic_id_list_element: None,
            type_value_list_element: type_value_list_element,
            value_type_list_element: None,
            value: None,
        }
    }

    pub fn add_submodel_element(&mut self, element: SubmodelElement) {
        if let Some(v) = self.value.as_mut() {
            v.push(element);
        } else {
            self.value = Some(vec![element]);
        }
    }
}
