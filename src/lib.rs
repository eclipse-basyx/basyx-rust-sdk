// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

//! # A Rust library to work with Asset Administration Shells (AAS)
//!
//! # Examples
//!
//! ## Creating a Submodel and Serialize to JSON
//! ```no_run
//! use basyx_rs::{DataTypeDefXsd, id_short_from_str};
//! use basyx_rs::prelude::*;
//! use std::io::Write;
//!
//! let mut property = Property::new(DataTypeDefXsd::XsBoolean);
//! property.category = Some(format!("{}", Category::CONSTANT));
//!
//! // id_short mandatory for all non-identifiable referables.
//! if let Some(id_short) = id_short_from_str("my_property1").ok() {
//!         property.id_short = Some(id_short);
//!    }
//!
//! let sme = SubmodelElement::Property(property);
//!
//! let mut submodel = Submodel::new("https://example.com/ids/1234567890".to_string());
//!
//! submodel.add_submodel_element(sme.clone());
//!
//! let json = serde_json::to_vec(&submodel);
//! let mut file = std::fs::OpenOptions::new()
//!     .create(true)
//!     .write(true)
//!     .read(true)
//!     .truncate(true)
//!     .open("submodel1.json");
//! let json = json.unwrap();
//! let buf: &[u8] = &json; // impl Deref for Vec<T> with Target = [T]
//! file.unwrap().write(buf);
//! ```

#![forbid(unsafe_code)]

mod administrative_information;
mod asset_administration_shell;
mod asset_information;
mod asset_kind;
mod category;
mod concept_description;
mod data_object_type_name;
mod data_specification_content;
mod data_specification_iec61360;
mod data_type_def_xsd;
mod data_type_iec61360;
mod embedded_data_specification;
mod environment;
mod extension;
mod id_short_from_string;
mod key;
mod key_types;
mod lang_string;
mod level_type;
mod modelling_kind;
pub mod prelude;
mod qualifier;
mod qualifier_kind;
mod reference;
mod reference_types;
mod resource;
mod specific_asset_id;
mod submodel;
pub mod submodel_element;
mod value_list;
mod value_reference_pair;

pub use administrative_information::AdministrativeInformation;
pub use asset_administration_shell::AssetAdministrationShell;
pub use asset_information::AssetInformation;
pub use asset_kind::AssetKind;
pub use category::Category;
pub use concept_description::ConceptDescription;
pub use data_object_type_name::DataObjectTypeName;
pub use data_specification_content::DataSpecificationContent;
pub use data_specification_iec61360::DataSpecificationIec61360;
pub use data_type_def_xsd::DataTypeDefXsd;
pub use data_type_iec61360::DataTypeIec61360;
pub use embedded_data_specification::EmbeddedDataSpecification;
pub use environment::Environment;
pub use extension::Extension;
pub use id_short_from_string::*;
pub use key::Key;
pub use key_types::KeyTypes;
pub use lang_string::LangString;
pub use level_type::LevelType;
pub use modelling_kind::ModellingKind;
pub use qualifier::Qualifier;
pub use qualifier_kind::QualifierKind;
pub use reference::Reference;
pub use reference_types::ReferenceTypes;
pub use resource::Resource;
pub use specific_asset_id::SpecificAssetId;
pub use submodel::Submodel;
pub use value_list::ValueList;
pub use value_reference_pair::ValueReferencePair;
