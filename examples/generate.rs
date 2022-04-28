// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

use basyx_rs::prelude::*;
use color_eyre::eyre::Result;
use std::io::Write;

fn main() -> Result<()> {
    let mut property = Property::new(
        "property".into(),
        Some(Value::Boolean(false)),
        DataObjectTypeName::Boolean,
    );
    property.category = Some(Category::CONSTANT);
    property.kind = Some(ModelingKind::Instance);

    let sme = SubmodelElement::SMProperty(property);

    let sm = Submodel::new("submodel1".into(), KeyType::IdShort, "i".into(), vec![sme]);
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
