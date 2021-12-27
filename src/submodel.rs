// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use crate::administrative_information::AdministrativeInformation;
use crate::category::Category;
use crate::embedded_data_specification::EmbeddedDataSpecification;
use crate::identifier::Identifier;
use crate::key_type::KeyType;
use crate::model_type::{ModelType, ModelTypeName};
use crate::modeling_kind::ModelingKind;
use crate::reference::Reference;
use crate::submodel_element::{Qualifier, SubmodelElement};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Submodel {
    // Submodel
    pub submodel_elements: Vec<SubmodelElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ModelingKind>,

    // Identifiable
    pub identification: Identifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administration: Option<AdministrativeInformation>,

    // Referable
    pub id_short: String,
    pub model_type: ModelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,

    // HasDataSpecification
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embedded_data_specification: Vec<EmbeddedDataSpecification>,

    // Qualifiable
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub qualifiers: Vec<Qualifier>,

    // HasSemantics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_id: Option<Reference>,
}

impl Submodel {
    pub fn new(
        id: String,
        id_type: KeyType,
        id_short: String,
        submodel_elements: Vec<SubmodelElement>,
    ) -> Self {
        Self {
            id_short,
            submodel_elements,
            semantic_id: None,
            qualifiers: vec![],
            embedded_data_specification: vec![],
            identification: Identifier { id_type, id },
            administration: None,
            category: None,
            model_type: ModelType::new(ModelTypeName::Submodel),
            kind: None,
        }
    }
}
