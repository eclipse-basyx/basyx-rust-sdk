// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DataObjectTypeName {
    Boolean,
    Double,
    Int,
    Integer,
    AnyType,
    ComplexType,
    Date,
    DateTime,
    Decimal,
    Short,
    Byte,
    Duration,
    String,
    Time,
    HexBinary,
    Float,
}

impl DataObjectTypeName {
    pub fn get(value_type: String) -> DataObjectTypeName {
        match value_type.as_str().to_lowercase().as_str() {
            "boolean" => DataObjectTypeName::Boolean,
            "double" => DataObjectTypeName::Double,
            "int" => DataObjectTypeName::Int,
            "short" => DataObjectTypeName::Short,
            "byte" => DataObjectTypeName::Byte,
            "string" => DataObjectTypeName::String,
            "time" => DataObjectTypeName::Time,
            "duration" => DataObjectTypeName::Duration,
            "decimal" => DataObjectTypeName::Decimal,
            "hexbinary" => DataObjectTypeName::HexBinary,
            "datetime" => DataObjectTypeName::DateTime,
            "date" => DataObjectTypeName::Date,
            "complextype" => DataObjectTypeName::ComplexType,
            "anytype" => DataObjectTypeName::AnyType,
            "integer" => DataObjectTypeName::Integer,
            "float" => DataObjectTypeName::Float,
            _ => DataObjectTypeName::default(),
        }
    }
}

impl Default for DataObjectTypeName {
    fn default() -> Self {
        DataObjectTypeName::Boolean
    }
}

impl ToString for DataObjectTypeName {
    fn to_string(&self) -> std::string::String {
        let s = match self {
            DataObjectTypeName::Boolean => "boolean",
            DataObjectTypeName::Double => "double",
            DataObjectTypeName::Int => "int",
            DataObjectTypeName::Short => "short",
            DataObjectTypeName::Byte => "byte",
            DataObjectTypeName::String => "string",
            DataObjectTypeName::Time => "time",
            DataObjectTypeName::Duration => "duration",
            DataObjectTypeName::Decimal => "decimal",
            DataObjectTypeName::HexBinary => "hexBinary",
            DataObjectTypeName::DateTime => "dateTime",
            DataObjectTypeName::Date => "date",
            DataObjectTypeName::ComplexType => "complexType",
            DataObjectTypeName::AnyType => "anyType",
            DataObjectTypeName::Integer => "integer",
            DataObjectTypeName::Float => "float",
        };
        s.to_string()
    }
}
