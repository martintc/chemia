use serde::{Serialize, Deserialize};

use super::package_builds::PackageBuilds;

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    id: Option<u64>,

    name: Option<String>,

    projectname: Option<String>,

    ownername: Option<String>,

    source_type: Option<String>,

    source_dict: Option<String>,

    auto_rebuild: Option<bool>,

    builds: Vec<PackageBuilds>
}