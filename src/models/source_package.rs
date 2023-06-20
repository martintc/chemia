use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SourcePackage {
    name: Option<String>,

    url: Option<String>,

    version: Option<String>,
}
