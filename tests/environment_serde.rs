// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use basyx_rs::prelude::SubmodelElement;
use basyx_rs::submodel_element::SubmodelElementCollection;
use basyx_rs::submodel_element::{File, Property};
use basyx_rs::{
    id_short_from_str, AdministrativeInformation, AssetAdministrationShell, AssetInformation,
    AssetKind, Category, ConceptDescription, DataSpecificationIec61360, DataTypeDefXsd,
    DataTypeIec61360, EmbeddedDataSpecification, Environment, Key, KeyTypes, LangString,
    ModellingKind, Reference, ReferenceTypes, Resource, SpecificAssetId, Submodel,
};
use color_eyre::eyre::Result;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

#[test]
fn environment_example_simple() {
    // arrange
    let mut asset_information = AssetInformation::new(AssetKind::Instance);
    asset_information.global_asset_id = Some("http://customer.com/assets/KHBVZJSQKIY".to_string());

    let specific_asset_id_equipment = SpecificAssetId {
        semantic_id: None,
        supplemental_semantic_ids: None,
        name: "EquipmentID".to_string(),
        value: "538fd1b3-f99f-4a52-9c75-72e9fa921270".to_string(),
        external_subject_id: Some(Reference::new(
            ReferenceTypes::ExternalReference,
            Key::new(
                KeyTypes::GlobalReference,
                "http://customer.com/Systems/ERP/012".to_string(),
            ),
        )),
    };

    let specific_asset_id_device = SpecificAssetId {
        semantic_id: None,
        supplemental_semantic_ids: None,
        name: "DeviceID".to_string(),
        value: "QjYgPggjwkiHk4RrQiYSLg==".to_string(),
        external_subject_id: Some(Reference::new(
            ReferenceTypes::ExternalReference,
            Key::new(
                KeyTypes::GlobalReference,
                "http://customer.com/Systems/IoT/1".to_string(),
            ),
        )),
    };

    asset_information.specific_asset_ids =
        Some(vec![specific_asset_id_equipment, specific_asset_id_device]);

    asset_information.default_thumbnail = Some(Resource {
        path: "file:///master/verwaltungsschale-detail-part1.png".to_string(),
        content_type: Some("image/png".to_string()),
    });

    let mut aas = AssetAdministrationShell::new(
        "http://customer.com/aas/9175_7013_7091_9168".to_string(),
        asset_information,
    );

    aas.id_short = id_short_from_str("ExampleMotor").ok();

    let mut sm_technical_data =
        Submodel::new("http://i40.customer.com/type/1/1/7A7104BDAB57E184".into());
    sm_technical_data.id_short = id_short_from_str("TechnicalData").ok();
    sm_technical_data.semantic_id = Some(Reference::new(
        ReferenceTypes::ExternalReference,
        Key::new(
            KeyTypes::GlobalReference,
            "0173-1#01-AFZ615#016".to_string(),
        ),
    ));

    let mut prop = Property::new(DataTypeDefXsd::XsInteger);
    prop.semantic_id = Some(Reference::new(
        ReferenceTypes::ExternalReference,
        Key::new(
            KeyTypes::ConceptDescription,
            "0173-1#02-BAA120#008".to_string(),
        ),
    ));
    prop.value = Some("5000".to_string());
    prop.category = Some(Category::PARAMETER.to_string());
    prop.id_short = id_short_from_str("MaxRotationSpeed").ok();

    sm_technical_data.submodel_elements = Some(vec![SubmodelElement::Property(prop)]);

    let mut sm_documentation =
        Submodel::new("http://i40.customer.com/type/1/1/1A7B62B529F19152".into());
    sm_documentation.id_short = id_short_from_str("Documentation").ok();
    sm_documentation.kind = Some(ModellingKind::Instance);

    let mut smc_operating_manual = SubmodelElementCollection {
        extensions: None,
        category: None,
        id_short: id_short_from_str("OperatingManual").ok(),
        display_name: None,
        description: None,
        semantic_id: Some(Reference::new(
            ReferenceTypes::ExternalReference,
            Key::new(
                KeyTypes::ConceptDescription,
                "http://www.vdi2770.com/blatt1/Entwurf/Okt18/cd/Document".to_string(),
            ),
        )),
        supplemental_semantic_ids: None,
        qualifiers: None,
        embedded_data_specifications: None,
        value: None,
    };

    smc_operating_manual.add_submodel_element(SubmodelElement::Property(Property {
        extensions: None,
        category: None,
        id_short: id_short_from_str("Title").ok(),
        display_name: None,
        description: None,
        semantic_id: Some(Reference::new(
            ReferenceTypes::ExternalReference,
            Key::new(
                KeyTypes::ConceptDescription,
                "http://www.vdi2770.com/blatt1/Entwurf/Okt18/cd/Description/Title".to_string(),
            ),
        )),
        supplemental_semantic_ids: None,
        qualifiers: None,
        embedded_data_specifications: None,
        value: Some("OperatingManual".to_string()),
        value_id: None,
        value_type: DataTypeDefXsd::XsString,
    }));

    smc_operating_manual.add_submodel_element(SubmodelElement::File(File {
        extensions: None,
        category: None,
        id_short: id_short_from_str("DigitalFile_PDF").ok(),
        display_name: None,
        description: None,
        semantic_id: Some(Reference::new(
            ReferenceTypes::ExternalReference,
            Key::new(
                KeyTypes::ConceptDescription,
                "http://www.vdi2770.com/blatt1/Entwurf/Okt18/cd/StoredDocumentRepresentation/DigitalFile".to_string(),
            ),
        )),
        supplemental_semantic_ids: None,
        qualifiers: None,
        embedded_data_specifications: None,
        value: Some("file:///aasx/OperatingManual.pdf".to_string()),
        content_type: "application/pdf".to_string(),
    }));

    sm_documentation.add_submodel_element(SubmodelElement::SubmodelElementCollection(
        smc_operating_manual,
    ));

    let mut sm_operational_data =
        Submodel::new("http://i40.customer.com/instance/1/1/AC69B1CB44F07935".into());
    sm_operational_data.id_short = id_short_from_str("OperationalData").ok();

    sm_operational_data.kind = Some(ModellingKind::Instance);

    sm_operational_data.add_submodel_element(SubmodelElement::Property(Property {
        extensions: None,
        category: Some(Category::VARIABLE.to_string()),
        id_short: id_short_from_str("RotationSpeed").ok(),
        display_name: None,
        description: None,
        semantic_id: Some(Reference::new(
            ReferenceTypes::ExternalReference,
            Key::new(
                KeyTypes::ConceptDescription,
                "http://customer.com/cd/1/1/18EBD56F6B43D895".to_string(),
            ),
        )),
        supplemental_semantic_ids: None,
        qualifiers: None,
        embedded_data_specifications: None,
        value: Some("4370".to_string()),
        value_id: None,
        value_type: DataTypeDefXsd::XsInteger,
    }));

    // add a reference of each submodel to the aas
    aas.add_reference_to_submodel(&sm_technical_data, ReferenceTypes::ExternalReference, false);
    // References in input file point somewhere else
    // aas.add_reference_to_submodel(
    //     &sm_documentation,
    //     ReferenceTypes::ExternalReference,
    //     false);
    // aas.add_reference_to_submodel(
    //     &sm_operational_data,
    //     ReferenceTypes::ExternalReference,
    //     false,
    // );
    if let Some(submodel_vec) = aas.submodels.as_mut() {
        submodel_vec.push(Reference {
            type_: ReferenceTypes::ExternalReference,
            referred_semantic_id: None,
            keys: vec![Key::new(
                KeyTypes::Submodel,
                "http://i40.customer.com/instance/1/1/AC69B1CB44F07935".to_string(),
            )],
        });

        submodel_vec.push(Reference {
            type_: ReferenceTypes::ExternalReference,
            referred_semantic_id: None,
            keys: vec![Key::new(
                KeyTypes::Submodel,
                "http://i40.customer.com/type/1/1/1A7B62B529F19152".to_string(),
            )],
        });
    }

    // *** Concept Descriptions ***
    let mut cd_title = ConceptDescription::new(
        "http://www.vdi2770.com/blatt1/Entwurf/Okt18/cd/Description/Title".to_string(),
    );
    cd_title.id_short = id_short_from_str("Title").ok();
    let embedded_data_spec1 = EmbeddedDataSpecification {
        data_specification: Reference {
            type_: ReferenceTypes::ExternalReference,
            referred_semantic_id: None,
            keys: vec![Key::new(
                KeyTypes::GlobalReference,
                "https://admin-shell.io/aas/3/0/RC02/DataSpecificationIEC61360".to_string(),
            )],
        },
        data_specification_content: DataSpecificationIec61360 {
            data_type: Some(DataTypeIec61360::StringTranslatable),
            definition: Some(vec![LangString::new(
                "EN".to_string(),
                "SprachabhängigerTiteldesDokuments.".to_string(),
            )]),
            level_type: None,
            preferred_name: vec![
                LangString::new("EN".to_string(), "Title".to_string()),
                LangString::new("DE".to_string(), "Titel".to_string()),
            ],
            short_name: Some(vec![
                LangString::new("EN".to_string(), "Title".to_string()),
                LangString::new("DE".to_string(), "Titel".to_string()),
            ]),
            source_of_definition: Some("ExampleString".to_string()),
            symbol: None,
            unit: Some("ExampleString".to_string()),
            unit_id: None,
            value: None,
            value_format: None,
            value_list: None,
        },
    };
    cd_title.embedded_data_specifications = Some(vec![embedded_data_spec1]);

    // Second ConceptDescription
    let mut cd_digital_file = ConceptDescription::new(
        "http://www.vdi2770.com/blatt1/Entwurf/Okt18/cd/StoredDocumentRepresentation/DigitalFile"
            .to_string(),
    );
    cd_digital_file.id_short = id_short_from_str("DigitalFile").ok();
    let embedded_data_spec2 = EmbeddedDataSpecification {
        data_specification: Reference {
            type_: ReferenceTypes::ExternalReference,
            referred_semantic_id: None,
            keys: vec![Key::new(
                KeyTypes::GlobalReference,
                "https://admin-shell.io/aas/3/0/RC02/DataSpecificationIEC61360".to_string(),
            )],
        },
        data_specification_content: DataSpecificationIec61360 {
            data_type: Some(DataTypeIec61360::String),
            definition: Some(vec![LangString::new(
                "EN".to_string(),
                "A file representing the document version. In addition to the mandatory PDF file, other files can be specified.".to_string(),
            )]),
            level_type: None,
            preferred_name: vec![
                LangString::new("EN".to_string(), "DigitalFile".to_string()),
                LangString::new("EN".to_string(), "DigitalFile".to_string()), // fault in original file
            ],
            short_name: Some(vec![
                LangString::new("EN".to_string(), "DigitalFile".to_string()),
                LangString::new("DE".to_string(), "DigitaleDatei".to_string()),
            ]),
            source_of_definition: Some("ExampleString".to_string()),
            symbol: None,
            unit: Some("ExampleString".to_string()),
            unit_id: None,
            value: None,
            value_format: None,
            value_list: None,
        },
    };
    cd_digital_file.embedded_data_specifications = Some(vec![embedded_data_spec2]);

    // Third ConceptDescription
    let mut cd_max_rotationspeed = ConceptDescription::new("0173-1#02-BAA120#008".to_string());
    cd_max_rotationspeed.id_short = id_short_from_str("MaxRotationSpeed").ok();
    let embedded_data_spec3 = EmbeddedDataSpecification {
        data_specification: Reference {
            type_: ReferenceTypes::ExternalReference,
            referred_semantic_id: None,
            keys: vec![Key::new(
                KeyTypes::GlobalReference,
                "https://admin-shell.io/aas/3/0/RC02/DataSpecificationIEC61360".to_string(),
            )],
        },
        data_specification_content: DataSpecificationIec61360 {
            data_type: Some(DataTypeIec61360::RealMeasure),
            definition: Some(vec![
                LangString::new(
                    "de".to_string(),
                    "HöchstezulässigeDrehzahl,mitwelcherderMotoroderdieSpeiseinheitbetriebenwerdendarf".to_string()),
                LangString::new(
                    "EN".to_string(),
                    "Greatestpermissiblerotationspeedwithwhichthemotororfeedingunitmaybeoperated".to_string())]),
            level_type: None,
            preferred_name: vec![
                LangString::new("de".to_string(), "max.Drehzahl".to_string()),
                LangString::new("en".to_string(), "Max.rotationspeed".to_string()),
            ],
            short_name: None,
            source_of_definition: Some("ExampleString".to_string()),
            symbol: None,
            unit: Some("1/min".to_string()),
            unit_id: Some(Reference{
                type_: ReferenceTypes::ExternalReference,
                referred_semantic_id: None,
                keys: vec![Key::new(KeyTypes::GlobalReference, "0173-1#05-AAA650#002".to_string())],
            }),
            value: None,
            value_format: None,
            value_list: None,
        },
    };
    cd_max_rotationspeed.administration = Some(AdministrativeInformation {
        embedded_data_specifications: None,
        version: Some("2".to_string()),
        revision: Some("1".to_string()),
        creator: None,
        template_id: None,
    });
    cd_max_rotationspeed.category = Some("PROPERTY".to_string()); // Is Property still in Category?
    cd_max_rotationspeed.embedded_data_specifications = Some(vec![embedded_data_spec3]);

    // Fourth ConceptDescription
    let mut cd_actual_rotationspeed =
        ConceptDescription::new("http://customer.com/cd/1/1/18EBD56F6B43D895".to_string());
    cd_actual_rotationspeed.id_short = id_short_from_str("RotationSpeed").ok();
    let embedded_data_spec4 = EmbeddedDataSpecification {
        data_specification: Reference {
            type_: ReferenceTypes::ExternalReference,
            referred_semantic_id: None,
            keys: vec![Key::new(
                KeyTypes::GlobalReference,
                "https://admin-shell.io/aas/3/0/RC02/DataSpecificationIEC61360".to_string(),
            )],
        },
        data_specification_content: DataSpecificationIec61360 {
            data_type: Some(DataTypeIec61360::RealMeasure),
            definition: Some(vec![
                LangString::new(
                    "DE".to_string(),
                    "Aktuelle Drehzahl, mitwelcher der Motor oder die Speiseinheit betrieben wird"
                        .to_string(),
                ),
                LangString::new(
                    "EN".to_string(),
                    "Actual rotationspeed with which the motor or feedingunit is operated"
                        .to_string(),
                ),
            ]),
            level_type: None,
            preferred_name: vec![
                LangString::new("DE".to_string(), "AktuelleDrehzahl".to_string()),
                LangString::new("EN".to_string(), "Actualrotationspeed".to_string()),
            ],
            short_name: Some(vec![
                LangString::new("DE".to_string(), "AktuelleDrehzahl".to_string()),
                LangString::new("EN".to_string(), "ActRotationSpeed".to_string()),
            ]),
            source_of_definition: Some("ExampleString".to_string()),
            symbol: None,
            unit: Some("1/min".to_string()),
            unit_id: Some(Reference {
                type_: ReferenceTypes::ExternalReference,
                referred_semantic_id: None,
                keys: vec![Key::new(
                    KeyTypes::GlobalReference,
                    "0173-1#05-AAA650#002".to_string(),
                )],
            }),
            value: None,
            value_format: None,
            value_list: None,
        },
    };
    cd_actual_rotationspeed.category = Some("PROPERTY".to_string()); // Is Property still in Category?
    cd_actual_rotationspeed.embedded_data_specifications = Some(vec![embedded_data_spec4]);

    // Fifth ConceptDescription
    let mut cd_document = ConceptDescription::new(
        "http://www.vdi2770.com/blatt1/Entwurf/Okt18/cd/Document".to_string(),
    );
    cd_document.id_short = id_short_from_str("Document").ok();
    let embedded_data_spec5 = EmbeddedDataSpecification {
        data_specification: Reference {
            type_: ReferenceTypes::ExternalReference,
            referred_semantic_id: None,
            keys: vec![Key::new(
                KeyTypes::GlobalReference,
                "https://admin-shell.io/aas/3/0/RC02/DataSpecificationIEC61360".to_string(),
            )],
        },
        data_specification_content: DataSpecificationIec61360 {
            data_type: Some(DataTypeIec61360::String),
            definition: Some(vec![LangString::new(
                "EN".to_string(),
                "Feste und geordnete Menge von für die Verwendung durch Personen bestimmte Informationen, die verwaltet und als Einheit zwischen Benutzern und System ausgetauscht werden kann.".to_string(),
            )]),
            level_type: None,
            preferred_name: vec![
                LangString::new("EN".to_string(), "Document".to_string())
            ],
            short_name: Some(vec![
                LangString::from_str("\"Document\"@EN").ok().unwrap(),
                LangString::from_str("\"Dokument\"@DE").ok().unwrap(),
            ]),
            source_of_definition: Some("[ISO15519-1:2010]".to_string()),
            symbol: None,
            unit: Some("ExampleString".to_string()),
            unit_id: None,
            value: None,
            value_format: None,
            value_list: None,
        },
    };
    cd_document.embedded_data_specifications = Some(vec![embedded_data_spec5]);

    // *** create the environment and add all elements ***
    let mut environment = Environment::default();
    environment.asset_administration_shells = Some(vec![aas]);
    environment.concept_descriptions = Some(vec![
        cd_title,
        cd_digital_file,
        cd_max_rotationspeed,
        cd_actual_rotationspeed,
        cd_document,
    ]);
    environment.submodels = Some(vec![
        sm_technical_data,
        sm_documentation,
        sm_operational_data,
    ]);

    // act
    //let utjson = serde_json::to_value(&environment).unwrap();
    let expected = read_json_from_file(Path::new(
        r"C:\Users\janhe\source\repos\rust\basyx-rust-sdk\tests\Example-Simple.json",
    ))
    .unwrap();

    // assert
    assert_eq!(environment, expected);
}

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Environment> {
    // Open the file in read-only mode with buffer.
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Environment`.
    let environment = serde_json::from_reader(reader)?;

    // Return the `Environment`.
    Ok(environment)
}
