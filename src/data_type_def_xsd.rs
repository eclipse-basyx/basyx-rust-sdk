// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(EnumString)]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum DataTypeDefXsd {
    #[serde(rename = "xs:anyURI")]
    XsAnyURI,
    #[serde(rename = "xs:base64Binary")]
    XsBase64Binary,
    #[serde(rename = "xs:boolean")]
    XsBoolean,
    #[serde(rename = "xs:byte")]
    XsByte,
    #[serde(rename = "xs:date")]
    XsDate,
    #[serde(rename = "xs:dateTime")]
    XsDateTime,
    #[serde(rename = "xs:decimal")]
    XsDecimal,
    #[serde(rename = "xs:double")]
    XsDouble,
    #[serde(rename = "xs:duration")]
    XsDuration,
    #[serde(rename = "xs:float")]
    XsFloat,
    #[serde(rename = "xs:gDay")]
    XsGDay,
    #[serde(rename = "xs:gMonth")]
    XsGMonth,
    #[serde(rename = "xs:gMonthDay")]
    XsGMonthDay,
    #[serde(rename = "xs:gYear")]
    XsGYear,
    #[serde(rename = "xs:gYearMonth")]
    XsGYearMonth,
    #[serde(rename = "xs:hexBinary")]
    XsHexBinary,
    #[serde(rename = "xs:int")]
    XsInt,
    #[serde(rename = "xs:integer")]
    XsInteger,
    #[serde(rename = "xs:long")]
    XsLong,
    #[serde(rename = "xs:negativeInteger")]
    XsNegativeInteger,
    #[serde(rename = "xs:nonNegativeInteger")]
    XsNonNegativeInteger,
    #[serde(rename = "xs:nonPositiveInteger")]
    XsNonPositiveInteger,
    #[serde(rename = "xs:positiveInteger")]
    XsPositiveInteger,
    #[serde(rename = "xs:short")]
    XsShort,
    #[serde(rename = "xs:string")]
    XsString,
    #[serde(rename = "xs:time")]
    XsTime,
    #[serde(rename = "xs:unsignedByte")]
    XsUnsignedByte,
    #[serde(rename = "xs:unsignedInt")]
    XsUnsignedInt,
    #[serde(rename = "xs:unsignedLong")]
    XsUnsignedLong,
    #[serde(rename = "xs:unsignedShort")]
    XsUnsignedShort,
}