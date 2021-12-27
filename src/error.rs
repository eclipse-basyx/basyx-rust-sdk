// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use crate::DataObjectTypeName;
use std::{
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AASError {
    #[error("{0}")]
    ParseBoolError(#[from] ParseBoolError),
    #[error("{0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("{0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Unsupported Type: {0:?}")]
    UnsupportedType(DataObjectTypeName),
}
