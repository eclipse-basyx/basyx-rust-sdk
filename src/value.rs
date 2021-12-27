// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use crate::{error::AASError, DataObjectTypeName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Value {
    Int(i32),
    String(String),
    Boolean(bool),
    Double(f64),
}

impl Value {
    pub fn new(value_type: DataObjectTypeName, value: String) -> Result<Option<Self>, AASError> {
        if value == "null" {
            Ok(None)
        } else {
            let value = match value_type {
                DataObjectTypeName::Int => Some(Value::Int(value.parse::<i32>()?)),
                DataObjectTypeName::Double => Some(Value::Double(value.parse::<f64>()?)),
                DataObjectTypeName::Float => Some(Value::Double(value.parse::<f64>()?)),
                DataObjectTypeName::String => Some(Value::String(value)),
                DataObjectTypeName::Short => Some(Value::Int(value.parse::<i32>()?)),
                DataObjectTypeName::Boolean => Some(Value::Boolean(value.parse::<bool>()?)),
                DataObjectTypeName::Byte => Some(Value::Int(value.parse::<i32>()?)),
                _ => return Err(AASError::UnsupportedType(value_type)),
            };
            Ok(value)
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> std::string::String {
        match &self {
            Value::Int(a) => format!("{}", a),
            Value::String(a) => a.clone(),
            Value::Boolean(a) => format!("{}", a),
            Value::Double(a) => format!("{}", a),
        }
    }
}
