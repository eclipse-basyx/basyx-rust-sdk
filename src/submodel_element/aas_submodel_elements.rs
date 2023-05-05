// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display, PartialEq)]
pub enum AasSubmodelElements {
    AnnotatedRelationshipElement,
    BasicEventElement,
    Blob,
    Capability,
    DataElement,
    Entity,
    EventElement,
    File,
    MultiLanguageProperty,
    Operation,
    Property,
    Range,
    ReferenceElement,
    RelationshipElement,
    SubmodelElement,
    SubmodelElementCollection,
    SubmodelElementList,
}
