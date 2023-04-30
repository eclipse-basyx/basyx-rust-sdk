// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use super::SubmodelElement;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct OperationVariable {
    pub value: SubmodelElement,
}

impl OperationVariable {
    pub fn new(submodel_element: SubmodelElement) -> Self {
        Self { value: submodel_element }
    }
}
