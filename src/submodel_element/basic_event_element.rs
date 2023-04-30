// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use super::EmbeddedDataSpecification;
use crate::{Extension, model_type::ModelType, reference::Reference, qualifier::Qualifier};
use serde::{Deserialize, Serialize};
use crate::LangString as LangStringNameType;
use crate::LangString as LangStringTextType;
use crate::submodel_element::direction::Direction;
use crate::submodel_element::state_of_event::StateOfEvent;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BasicEventElement {
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
    
    pub observed: Reference,
    
    pub direction: Direction,
    
    pub state: StateOfEvent,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messageTopic")]
    pub message_topic: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messageBroker")]
    pub message_broker: Option<Reference>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lastUpdate")]
    pub last_update: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minInterval")]
    pub min_interval: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxInterval")]
    pub max_interval: Option<String>,
}

impl BasicEventElement {
    pub fn new(observed: Reference, direction: Direction, state: StateOfEvent) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            model_type: ModelType::BasicEventElement,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            observed,
            direction,
            state,
            message_topic: None,
            message_broker: None,
            last_update: None,
            min_interval: None,
            max_interval: None,
        }
    }
}
