// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(EnumString, Display, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ModelType {
    AnnotatedRelationshipElement,
    AssetAdministrationShell,
    BasicEventElement,
    Blob,
    Capability,
    ConceptDescription,
    DataSpecificationIec61360,
    Entity,
    File,
    MultiLanguageProperty,
    Operation,
    Property,
    Range,
    ReferenceElement,
    RelationshipElement,
    Submodel,
    SubmodelElementCollection,
    SubmodelElementList,
}
