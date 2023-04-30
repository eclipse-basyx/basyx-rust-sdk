// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use basyx_rs::prelude::*;
use color_eyre::eyre::Result;
use std::io::Write;
use basyx_rs::{DataTypeDefXsd, id_short_from_str};

fn main() -> Result<()> {
    let mut property = Property::new(DataTypeDefXsd::XsBoolean);
    property.category = Some(format!("{}", Category::CONSTANT));
    property.id_short = Some(id_short_from_str("my_id_short").unwrap());

    let sme = SubmodelElement::SmeProperty(property);

    let mut sm = Submodel::new("my_id".to_string());

    sm.add_submodel_element(sme.clone());

    serialize("submodel1.json", &sm)
}

fn serialize(path: &str, submodel: &Submodel) -> Result<()> {
    let json = serde_json::to_vec(submodel)?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .truncate(true)
        .open(path)?;
    file.write_all(&json)?;

    println!("Generated: {}", path);
    Ok(())
}
