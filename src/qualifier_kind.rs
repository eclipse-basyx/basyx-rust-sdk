// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(EnumString, Display, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum QualifierKind {
    ConceptQualifier,
    TemplateQualifier,
    ValueQualifier,
}
