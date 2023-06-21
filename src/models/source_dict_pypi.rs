use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceDictPyPi {
    pypi_package_name: Option<String>,

    pypi_package_version: Option<String>,

    spec_generator: Option<String>,

    spec_template: Option<String>,

    python_versions: Vec<String>,
}
