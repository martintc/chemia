use serde::{Deserialize, Serialize};

use crate::models::repo;

#[derive(Serialize, Deserialize)]
pub struct ProjectChrootBuildConfig {
    chroot: Option<String>,

    repos: Vec<repo::Repo>,

    additional_repos: Option<String>,

    additional_packages: Option<String>,

    additional_modules: Option<String>,

    enable_net: Option<bool>,

    with_opts: Option<String>,

    without_opts: Option<String>,

    isolation: Option<String>,
}
