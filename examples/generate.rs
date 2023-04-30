// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use basyx_rs::prelude::*;
use color_eyre::eyre::Result;
use std::io::Write;
use basyx_rs::DataTypeDefXsd;

fn main() -> Result<()> {
    let mut property = Property::new(DataTypeDefXsd::XsBoolean);
    property.category = Some(format!("{}", Category::CONSTANT));

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
