// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use crate::key_types::KeyTypes;
use serde::{Deserialize, Serialize};

/// A key is a reference to an element by its ID.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Key {
    /// Denotes which kind of entity is referenced
    ///
    // If Key/type = GlobalReference, the key represents a
    // reference to a source that can be globally identified.

    // If Key/type = FragmentReference, the key represents
    // a bookmark or a similar local identifier within its parent
    // element as specified by the key that precedes this key.

    // In all other cases, the key references a model element
    // of the same or another Asset Administration Shell. The
    // name of the model element is explicitly listed.
    #[serde(rename = "type")]
    pub type_: KeyTypes,

    /// The key value, for example an IRDI or an URI
    pub value: String,
}

impl Key {
    pub fn new(type_: KeyTypes, value: String) -> Self {
        Self { type_, value }
    }
}
