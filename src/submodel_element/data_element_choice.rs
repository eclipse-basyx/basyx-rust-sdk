// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use crate::submodel_element::{Blob, File, MultiLanguageProperty, Property, Range, ReferenceElement};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataElementChoice {
    DeBlob(Blob),
    DeFile(File),
    DeMultiLanguageProperty(MultiLanguageProperty),
    DeProperty(Property),
    DeRange(Range),
    DeReferenceElement(ReferenceElement),
}