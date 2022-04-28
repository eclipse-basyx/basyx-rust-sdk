// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum KeyType {
    IRI,
    IRDI,
    IdShort,
    FragmentId,
    Custom,
}

impl KeyType {}

impl std::str::FromStr for KeyType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "IRI" => KeyType::IRI,
            "IRDI" => KeyType::IRDI,
            "IdShort" => KeyType::IdShort,
            "FragmentId" => KeyType::FragmentId,
            "Custom" => KeyType::Custom,
            _ => return Err(()),
        })
    }
}

impl Default for KeyType {
    fn default() -> KeyType {
        KeyType::IdShort
    }
}
