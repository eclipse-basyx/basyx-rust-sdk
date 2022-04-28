// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

mod basic_event;
mod data_object_type;
pub mod data_specification_content;
mod embedded_data_specification;
mod file;
mod operation;
mod operation_variable;
mod property;
mod qualifier;
mod range;
mod reference_element;
mod submodel_element_collection;
mod value_type;
mod variable;

pub use basic_event::BasicEvent;
pub use data_object_type::DataObjectType;
pub use embedded_data_specification::EmbeddedDataSpecification;
pub use file::File;
pub use operation::Operation;
pub use operation_variable::OperationVariable;
pub use property::Property;
pub use qualifier::Qualifier;
pub use range::Range;
pub use reference_element::ReferenceElement;
use serde::{Deserialize, Serialize};
pub use submodel_element_collection::SubmodelElementCollection;
pub use value_type::ValueType;
pub use variable::Variable;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SubmodelElement {
    SMCollection(SubmodelElementCollection),
    SMEvent(BasicEvent),
    SMFile(File),
    SMOperation(Operation),
    SMProperty(Property),
    SMRange(Range),
    SMReferenceElement(ReferenceElement),
}

impl SubmodelElement {
    pub fn get_id_short(&self) -> String {
        match self {
            SubmodelElement::SMCollection(coll) => coll.id_short.clone(),
            SubmodelElement::SMEvent(coll) => coll.id_short.clone(),
            SubmodelElement::SMFile(coll) => coll.id_short.clone(),
            SubmodelElement::SMOperation(coll) => coll.id_short.clone(),
            SubmodelElement::SMProperty(coll) => coll.id_short.clone(),
            SubmodelElement::SMRange(coll) => coll.id_short.clone(),
            SubmodelElement::SMReferenceElement(coll) => coll.id_short.clone(),
        }
    }
}
