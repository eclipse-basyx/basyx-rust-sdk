// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};

#[derive(EnumString, Display)]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum KeyTypes {
    AnnotatedRelationshipElement,
    AssetAdministrationShell,
    BasicEventElement,
    Blob,
    Capability,
    ConceptDescription,
    DataElement,
    Entity,
    EventElement,
    File,
    FragmentReference,
    GlobalReference,
    Identifiable,
    MultiLanguageProperty,
    Operation,
    Property,
    Range,
    Referable,
    ReferenceElement,
    RelationshipElement,
    Submodel,
    SubmodelElement,
    SubmodelElementCollection,
    SubmodelElementList,
}