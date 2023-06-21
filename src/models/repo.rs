use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    baseurl: Option<String>,

    id: Option<String>,

    name: Option<String>,

    module_hotfixes: Option<bool>,
}
