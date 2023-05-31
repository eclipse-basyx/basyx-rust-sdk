// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use basyx_rs::{AssetAdministrationShell, AssetInformation, AssetKind, ReferenceTypes, Submodel};

fn setup() -> (Submodel, Submodel, Submodel) {
    let sm1 = Submodel::new("https://example.com/ids/1".to_string());
    let sm2 = Submodel::new("https://example.com/ids/2".to_string());
    let sm3 = Submodel::new("https://example.com/ids/3".to_string());
    (sm1, sm2, sm3)
}

#[test]
fn submodels_initially_none() {
    // arrange
    let ut = AssetAdministrationShell::new("".into(), AssetInformation::new(AssetKind::Instance));

    // act

    // assert
    assert_eq!(ut.submodels, None);
}

#[test]
fn add_one_gives_len_one() {
    // arrange
    let (sm1, _, _) = setup();
    let mut ut =
        AssetAdministrationShell::new("".into(), AssetInformation::new(AssetKind::Instance));

    // act
    ut.add_reference_to_submodel(&sm1, ReferenceTypes::ModelReference, true);

    // assert
    assert_eq!(
        ut.submodels
            .expect("AssetAdministrationShell/submodels is None")
            .len(),
        1
    );
}

#[test]
fn add_three_gives_len_three() {
    // arrange
    let (sm1, sm2, sm3) = setup();
    let mut ut =
        AssetAdministrationShell::new("".into(), AssetInformation::new(AssetKind::Instance));

    // act
    ut.add_reference_to_submodel(&sm1, ReferenceTypes::ModelReference, true);
    ut.add_reference_to_submodel(&sm2, ReferenceTypes::ModelReference, true);
    ut.add_reference_to_submodel(&sm3, ReferenceTypes::ModelReference, true);

    // assert
    assert_eq!(
        ut.submodels
            .expect("AssetAdministrationShell/submodels is None")
            .len(),
        3
    );
}

#[test]
fn add_three_and_delete_middle_gives_len_two() {
    // arrange
    let (sm1, sm2, sm3) = setup();
    let mut ut =
        AssetAdministrationShell::new("".into(), AssetInformation::new(AssetKind::Instance));

    ut.add_reference_to_submodel(&sm1, ReferenceTypes::ModelReference, true);
    ut.add_reference_to_submodel(&sm2, ReferenceTypes::ModelReference, true);
    ut.add_reference_to_submodel(&sm3, ReferenceTypes::ModelReference, true);

    // act
    ut.delete_reference_to_submodel(1);

    // assert
    assert_eq!(
        ut.submodels
            .expect("AssetAdministrationShell/submodels is None")
            .len(),
        2
    );
}
