// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::Parser;
use color_eyre::eyre::{anyhow, Context, Result};
use colored::Colorize;
use jsonschema::JSONSchema;
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Parser)]
#[clap(
    version = "0.1",
    author = "Fraunhofer Institute for Experimental Software Engineering IESE"
)]
struct Opts {
    #[clap(parse(from_os_str))]
    input: PathBuf,
    #[clap(short, long)]
    mode: Mode,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
enum Mode {
    AAS,
    Submodel,
}

impl FromStr for Mode {
    type Err = AASCheckError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "aas" => Ok(Mode::AAS),
            "submodel" => Ok(Mode::Submodel),
            _ => Err(AASCheckError::InvalidMode(s.to_string())),
        }
    }
}

#[derive(Error, Debug)]
pub enum AASCheckError {
    #[error("{0}")]
    InvalidMode(String),
}

fn main() -> Result<()> {
    let opt = Opts::parse();
    let instance = read_json(&opt.input)?;
    let schema = static_json()?;
    let compiled = JSONSchema::compile(&schema).map_err(|e| anyhow!(e.to_string()))?;
    check(&compiled, instance, opt.mode)?;
    output(&opt.input, opt.mode);
    Ok(())
}

fn check(schema: &JSONSchema, instance: Value, mode: Mode) -> Result<()> {
    let instance = match mode {
        Mode::AAS => instance,
        Mode::Submodel => {
            json!({
                "assetAdministrationShells": [],
                "submodels": [
                    instance
                ],
                "assets": [],
                "conceptDescriptions": []
            })
        }
    };
    let result = match schema.validate(&instance) {
        Ok(_) => Ok(()),
        Err(errors) => {
            let text = errors
                .into_iter()
                .map(|e| format!("{:#?}", e))
                .collect::<Vec<String>>()
                .join("\n");
            Err(color_eyre::eyre::anyhow!(text).wrap_err("Validation failed!"))
        }
    };
    result
}

fn read_json(path: &Path) -> Result<serde_json::Value> {
    let content =
        std::fs::read_to_string(path).wrap_err(format!("Opening file: {}", path.display()))?;
    Ok(serde_json::from_str(&content)?)
}

fn static_json() -> Result<serde_json::Value> {
    let content = include_str!("../../schema/aas.json");
    Ok(serde_json::from_str(content)?)
}

fn output(path: &Path, mode: Mode) {
    let t = match mode {
        Mode::AAS => "Asset Administration Shell",
        Mode::Submodel => "Submodel of an Asset Administration Shell",
    }
    .bold();
    let path = format!("{}", path.display()).bold();
    let text = format!("{} is a valid {}", path, t).green();
    println!("{}", text);
}
