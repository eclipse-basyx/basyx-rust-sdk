// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use super::{EmbeddedDataSpecification, Qualifier, SubmodelElement};
use crate::{
    category::Category,
    model_type::{ModelType, ModelTypeName},
    modeling_kind::ModelingKind,
    reference::Reference,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmodelElementCollection {
    // SubmodelElementCollection
    pub ordered: bool,
    pub allow_duplicates: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubmodelElement>,

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

impl SubmodelElementCollection {
    pub fn new(
        id_short: String,
        ordered: bool,
        allow_duplicates: bool,
        submodel_elements: Vec<SubmodelElement>,
    ) -> Self {
        Self {
            id_short,
            ordered,
            allow_duplicates,
            value: submodel_elements,
            semantic_id: None,
            embedded_data_specification: vec![],
            category: None,
            qualifiers: vec![],
            model_type: ModelType::new(ModelTypeName::SubmodelElementCollection),
            kind: None,
        }
    }
    pub fn add_submodel_element(&mut self, element: SubmodelElement) {
        self.value.push(element);
    }
}
