// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(EnumString)]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum DataTypeIec61360 {
    #[serde(rename = "BLOB")]
    Blob,
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "DATE")]
    Date,
    #[serde(rename = "FILE")]
    File,
    #[serde(rename = "HTML")]
    Html,
    #[serde(rename = "INTEGER_COUNT")]
    IntegerCount,
    #[serde(rename = "INTEGER_CURRENCY")]
    IntegerCurrency,
    #[serde(rename = "INTEGER_MEASURE")]
    IntegerMeasure,
    #[serde(rename = "IRDI")]
    Irdi,
    #[serde(rename = "IRI")]
    Iri,
    #[serde(rename = "RATIONAL")]
    Rational,
    #[serde(rename = "RATIONAL_MEASURE")]
    RationalMeasure,
    #[serde(rename = "REAL_COUNT")]
    RealCount,
    #[serde(rename = "REAL_CURRENCY")]
    RealCurrency,
    #[serde(rename = "REAL_MEASURE")]
    RealMeasure,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "STRING_TRANSLATABLE")]
    StringTranslatable,
    #[serde(rename = "TIME")]
    Time,
    #[serde(rename = "TIMESTAMP")]
    Timestamp,
}