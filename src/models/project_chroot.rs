use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectChroot {
    mock_chroot: Option<String>,

    ownername: Option<String>,

    projectname: Option<String>,

    comps_name: Option<String>,

    additional_repos: Vec<String>,

    additional_modules: Vec<String>,

    with_opts: Vec<String>,

    without_opts: Vec<String>,

    delete_after_days: Option<u32>,

    isolation: Option<String>
}