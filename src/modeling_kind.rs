use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModelingKind {
    Template,
    Instance,
}

impl FromStr for ModelingKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Instance" => ModelingKind::Instance,
            "Template" => ModelingKind::Template,
            _ => return Err(()),
        })
    }
}

impl Default for ModelingKind {
    fn default() -> Self {
        ModelingKind::Instance
    }
}
