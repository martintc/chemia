use serde::{Deserialize, Serialize};

use crate::models::source_package::SourcePackage;

#[derive(Serialize, Deserialize, Debug)]
pub struct Build {
    chroots: Vec<String>,

    ended_on: Option<u64>,

    id: Option<u64>,

    is_background: Option<bool>,

    owername: Option<String>,

    project_dirname: Option<String>,

    projectname: Option<String>,

    repo_url: Option<String>,

    started_on: Option<u64>,

    state: Option<String>,

    submitted_on: Option<u64>,

    submitter: Option<String>,

    #[serde(rename(deserialize = "source_package"))]
    source_package: Option<SourcePackage>,
}
