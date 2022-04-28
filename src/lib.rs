// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: EPL-2.0

//! # A Rust library to work with Asset Administration Shells (AAS)
//!
//! # Examples
//!
//! ## Creating a Submodel and Serialize to JSON
//! ```no_run
//! use basyx_rs::prelude::*;
//!
//! let mut property = Property::new(
//!     "property".into(),
//!     Some(Value::Boolean(false)),
//!     DataObjectTypeName::Boolean,
//! );
//! property.category = Some(Category::CONSTANT);
//! property.kind = Some(ModelingKind::Instance);
//!
//! let sme = SubmodelElement::SMProperty(property);

//! let submodel = Submodel::new("submodel1".into(), KeyType::IdShort, "i".into(), vec![sme]);
//!
//! let json = serde_json::to_vec(&submodel)?;
//! let mut file = std::fs::OpenOptions::new()
//!     .create(true)
//!     .write(true)
//!     .read(true)
//!     .truncate(true)
//!     .open(path"submodel1.json")?;
//! file.write_all(&json)?;
//! ```

#![forbid(unsafe_code)]

mod administrative_information;
mod asset;
mod asset_administration_shell;
mod asset_information;
mod asset_kind;
mod category;
mod concept_description;
mod data_object_type_name;
mod embedded_data_specification;
mod environment;
mod error;
mod identifier;
mod key;
mod key_type;
mod lang_string;
mod model_type;
mod modeling_kind;
pub mod prelude;
mod reference;
mod submodel;
pub mod submodel_element;
mod value;

pub use administrative_information::AdministrativeInformation;
pub use asset::Asset;
pub use asset_administration_shell::AssetAdministrationShell;
pub use asset_information::AssetInformation;
pub use asset_kind::AssetKind;
pub use category::Category;
pub use concept_description::ConceptDescription;
pub use data_object_type_name::DataObjectTypeName;
pub use environment::Environment;
pub use identifier::Identifier;
pub use key::Key;
pub use key_type::KeyType;
pub use lang_string::LangString;
pub use model_type::{ModelType, ModelTypeName};
pub use modeling_kind::ModelingKind;
pub use reference::Reference;
pub use submodel::Submodel;
pub use value::Value;
