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
#[serde(tag = "modelType")]
pub enum SubmodelElement {
    AnnotatedRelationshipElement(AnnotatedRelationshipElement),
    BasicEventElement(BasicEventElement),
    Blob(Blob),
    Capability(Capability),
    DataElement(DataElement),
    Entity(Entity),
    File(File),
    MultiLanguageProperty(MultiLanguageProperty),
    Operation(Operation),
    Property(Property),
    Range(Range),
    ReferenceElement(ReferenceElement),
    RelationshipElement(RelationshipElement),
    SubmodelElementCollection(SubmodelElementCollection),
    SubmodelElementList(SubmodelElementList),
}

impl SubmodelElement {
    pub fn get_id_short(&self) -> Option<String> {
        match self {
            SubmodelElement::AnnotatedRelationshipElement(element) => element.id_short.clone(),
            SubmodelElement::BasicEventElement(element) => element.id_short.clone(),
            SubmodelElement::Blob(element) => element.id_short.clone(),
            SubmodelElement::Capability(element) => element.id_short.clone(),
            SubmodelElement::DataElement(element) => element.id_short.clone(),
            SubmodelElement::Entity(element) => element.id_short.clone(),
            SubmodelElement::File(element) => element.id_short.clone(),
            SubmodelElement::MultiLanguageProperty(element) => element.id_short.clone(),
            SubmodelElement::Operation(element) => element.id_short.clone(),
            SubmodelElement::Property(element) => element.id_short.clone(),
            SubmodelElement::Range(element) => element.id_short.clone(),
            SubmodelElement::ReferenceElement(element) => element.id_short.clone(),
            SubmodelElement::RelationshipElement(element) => element.id_short.clone(),
            SubmodelElement::SubmodelElementCollection(element) => element.id_short.clone(),
            SubmodelElement::SubmodelElementList(element) => element.id_short.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        id_short_from_str, Category, DataTypeDefXsd, Key, KeyTypes, Reference, ReferenceTypes,
    };

    #[test]
    fn deserialize_submodel_element_property() {
        let input = r#"{
                    "modelType": "Property",
                    "semanticId":
                    {
                        "keys": [
                            {
                                "type": "ConceptDescription",
                                "value": "0173-1#02-BAA120#008"
                            }
                        ],
                        "type": "ExternalReference"
                    },
                    "value": "5000",
                    "valueType": "xs:integer",
                    "category": "PARAMETER",
                    "idShort": "MaxRotationSpeed"
                }"#;

        let ut: SubmodelElement = serde_json::from_str(input).unwrap();
        //let ut_enum: SubmodelElement = SubmodelElement::Property(ut);

        let mut prop = Property::new(DataTypeDefXsd::XsInteger);
        prop.semantic_id = Some(Reference::new(
            ReferenceTypes::ExternalReference,
            Key::new(
                KeyTypes::ConceptDescription,
                "0173-1#02-BAA120#008".to_string(),
            ),
        ));
        prop.value = Some("5000".to_string());
        prop.category = Some(Category::PARAMETER.to_string());
        prop.id_short = id_short_from_str("MaxRotationSpeed").ok();

        let sme_prop = SubmodelElement::Property(prop);

        assert_eq!(ut, sme_prop);
    }
}
