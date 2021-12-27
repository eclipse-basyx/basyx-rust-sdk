// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use super::OperationVariable;
use super::SubmodelElement;
use crate::model_type::{ModelType, ModelTypeName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    pub value: OperationVariable,
    pub model_type: ModelType,
}

impl Variable {
    pub fn new(submodel_element: SubmodelElement) -> Self {
        Self {
            value: OperationVariable { submodel_element },
            model_type: ModelType::new(ModelTypeName::OperationVariable),
        }
    }
}
