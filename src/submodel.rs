// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use crate::administrative_information::AdministrativeInformation;
use crate::embedded_data_specification::EmbeddedDataSpecification;
use crate::model_type::ModelType;
use crate::modelling_kind::ModellingKind;
use crate::qualifier::Qualifier;
use crate::reference::Reference;
use crate::submodel_element::SubmodelElement;
use crate::Extension;
use crate::LangString as LangStringNameType;
use crate::LangString as LangStringTextType;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Submodel {
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

    // HasKind
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ModellingKind>,

    // HasSemantics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "semanticId")]
    pub semantic_id: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supplementalSemanticIds")]
    pub supplemental_semantic_ids: Option<Vec<Reference>>,

    // Qualifiable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifiers: Option<Vec<Qualifier>>,

    // HasDataSpecification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "embeddedDataSpecifications")]
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "submodelElements")]
    pub submodel_elements: Option<Vec<SubmodelElement>>,
}

impl Submodel {
    pub fn new(id: String) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            id: id,
            kind: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            model_type: ModelType::Submodel,
            description: None,
            administration: None,
            submodel_elements: None,
        }
    }

    pub fn add_submodel_element(&mut self, element: SubmodelElement) {
        if let Some(v) = self.submodel_elements.as_mut() {
            v.push(element);
        } else {
            self.submodel_elements = Some(vec![element]);
        }
    }
}
