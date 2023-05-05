// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

mod aas_submodel_elements;
mod annotated_relationship_element;
mod basic_event_element;
mod blob;
mod capability;
mod data_element;
mod data_element_choice;
mod direction;
mod embedded_data_specification;
mod entity;
mod entity_type;
mod event_payload;
mod file;
mod multi_language_property;
mod operation;
mod operation_variable;
mod property;
mod range;
mod reference_element;
mod relationship_element;
mod state_of_event;
mod submodel_element_collection;
mod submodel_element_list;

pub use aas_submodel_elements::AasSubmodelElements;
pub use annotated_relationship_element::AnnotatedRelationshipElement;
pub use basic_event_element::BasicEventElement;
pub use blob::Blob;
pub use capability::Capability;
pub use data_element::DataElement;
pub use data_element_choice::DataElementChoice;
pub use embedded_data_specification::EmbeddedDataSpecification;
pub use entity::Entity;
pub use entity_type::EntityType;
pub use event_payload::EventPayload;
pub use file::File;
pub use multi_language_property::MultiLanguageProperty;
pub use operation::Operation;
pub use operation_variable::OperationVariable;
pub use property::Property;
pub use range::Range;
pub use reference_element::ReferenceElement;
pub use relationship_element::RelationshipElement;
pub use submodel_element_collection::SubmodelElementCollection;
pub use submodel_element_list::SubmodelElementList;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SubmodelElement {
    SmeAnnotatedRelationshipElement(AnnotatedRelationshipElement),
    SmeBasicEventElement(BasicEventElement),
    SmeBlob(Blob),
    SmeCapability(Capability),
    SmeDataElement(DataElement),
    SmeEntity(Entity),
    SmeFile(File),
    SmeMultiLanguageProperty(MultiLanguageProperty),
    SmeOperation(Operation),
    SmeProperty(Property),
    SmeRange(Range),
    SmeReferenceElement(ReferenceElement),
    SmeRelationshipElement(RelationshipElement),
    SmeSubmodelElementCollection(SubmodelElementCollection),
    SmeSubmodelElementList(SubmodelElementList),
}

impl SubmodelElement {
    pub fn get_id_short(&self) -> Option<String> {
        match self {
            SubmodelElement::SmeAnnotatedRelationshipElement(element) => element.id_short.clone(),
            SubmodelElement::SmeBasicEventElement(element) => element.id_short.clone(),
            SubmodelElement::SmeBlob(element) => element.id_short.clone(),
            SubmodelElement::SmeCapability(element) => element.id_short.clone(),
            SubmodelElement::SmeDataElement(element) => element.id_short.clone(),
            SubmodelElement::SmeEntity(element) => element.id_short.clone(),
            SubmodelElement::SmeFile(element) => element.id_short.clone(),
            SubmodelElement::SmeMultiLanguageProperty(element) => element.id_short.clone(),
            SubmodelElement::SmeOperation(element) => element.id_short.clone(),
            SubmodelElement::SmeProperty(element) => element.id_short.clone(),
            SubmodelElement::SmeRange(element) => element.id_short.clone(),
            SubmodelElement::SmeReferenceElement(element) => element.id_short.clone(),
            SubmodelElement::SmeRelationshipElement(element) => element.id_short.clone(),
            SubmodelElement::SmeSubmodelElementCollection(element) => element.id_short.clone(),
            SubmodelElement::SmeSubmodelElementList(element) => element.id_short.clone(),
        }
    }
}
