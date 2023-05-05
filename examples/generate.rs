// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use basyx_rs::prelude::*;
use basyx_rs::{
    id_short_from_str, AssetAdministrationShell, AssetInformation, AssetKind, DataTypeDefXsd,
};
use color_eyre::eyre::Result;
use std::io::Write;

fn main() -> Result<()> {
    let mut property = Property::new(DataTypeDefXsd::XsBoolean);
    property.category = Some(format!("{}", Category::CONSTANT));

    if let Some(id_short) = id_short_from_str("my_property1").ok() {
        property.id_short = Some(id_short);
    }

    property.value = Some("true".to_string());

    let sme = SubmodelElement::SmeProperty(property);

    let mut sm = Submodel::new("https://example.com/ids/1234567890".to_string());

    if let Some(id_short) = id_short_from_str("my_submodel1").ok() {
        sm.id_short = Some(id_short);
    }

    sm.add_submodel_element(sme.clone());

    let mut aas = AssetAdministrationShell::new(
        "https://example.com/ids/0987654321".to_string(),
        AssetInformation::new(AssetKind::Instance),
    );

    if let Some(id_short) = id_short_from_str("my_aas1").ok() {
        aas.id_short = Some(id_short);
    }
    // unfortunately, aascheck won't notice, if the id_short is malformed.
    //aas.id_short = Some("_id_short".to_string());

    aas.add_reference_to_submodel(&sm);

    serialize("submodel1.json", &sm).ok();
    serialize("aas1.json", &aas)
}

fn serialize<T: serde::ser::Serialize>(path: &str, element: &T) -> Result<()> {
    let json = serde_json::to_vec(element)?;
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
