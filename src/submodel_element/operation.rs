// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use super::EmbeddedDataSpecification;
use crate::submodel_element::OperationVariable;
use crate::LangString as LangStringNameType;
use crate::LangString as LangStringTextType;
use crate::{model_type::ModelType, reference::Reference, Extension, Qualifier};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Operation {
    // Referable
    // HasExtension
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

    // SME Operation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inputVariables")]
    pub input_variables: Option<Vec<OperationVariable>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "outputVariables")]
    pub output_variables: Option<Vec<OperationVariable>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inoutputVariables")]
    pub inoutput_variables: Option<Vec<OperationVariable>>,
}

impl Operation {
    pub fn new() -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            model_type: ModelType::Operation,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            input_variables: None,
            output_variables: None,
            inoutput_variables: None,
        }
    }
}

impl Default for Operation {
    fn default() -> Self {
        Operation::new()
    }
}
