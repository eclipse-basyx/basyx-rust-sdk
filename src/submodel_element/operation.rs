// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use super::{variable::Variable, EmbeddedDataSpecification};
use crate::{
    category::Category,
    model_type::{ModelType, ModelTypeName},
    modeling_kind::ModelingKind,
    reference::Reference,
    submodel_element::Qualifier,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    // Operation
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub input_variable: Vec<Variable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub output_variable: Vec<Variable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inoutput_variable: Vec<Variable>,

    // SubmodelElement
    pub id_short: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ModelingKind>,

    // Referable
    pub model_type: ModelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,

    // HasDataSpecification
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embedded_data_specification: Vec<EmbeddedDataSpecification>,

    // HasSemantics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_id: Option<Reference>,

    // Qualifiable
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub qualifiers: Vec<Qualifier>,
}

impl Operation {
    pub fn new(
        id_short: String,
        input_variable: Vec<Variable>,
        output_variable: Vec<Variable>,
    ) -> Self {
        Self {
            id_short,
            model_type: ModelType::new(ModelTypeName::Operation),
            input_variable,
            output_variable,
            inoutput_variable: vec![],
            semantic_id: None,
            embedded_data_specification: vec![],
            category: None,
            kind: None,
            qualifiers: vec![],
        }
    }
}
