use serde::{Deserialize, Serialize};

use crate::models::build::Build;

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageBuilds {
    latest: Vec<Build>,

    latest_succeeded: Vec<Build>,
}
