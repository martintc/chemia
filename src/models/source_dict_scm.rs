use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceDictSCM {
    clone_url: Option<String>,

    committish: Option<String>,

    source_build_method: Option<String>,

    spec: Option<String>,

    subdirectory: Option<String>,

    #[serde(rename(deserialize = "type"))]
    t: Option<String>,
}
