use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename(deserialize = "copr-cli"))]
    coprcli: Option<CoprCli>
}

#[derive(Serialize, Deserialize, Debug)]
struct CoprCli {
    login: Option<String>,
    username: Option<String>,
    token: Option<String>,
    copr_url: Option<String>,
}

fn get_configuration() -> Data {
    let mut home_dir = simple_home_dir::home_dir().expect("Could not find home path!");
    home_dir.push(".config");
    home_dir.push("copr");

    if !home_dir.exists() {
        panic!("The configuration file does not exist!");
    }

    let file_contents = std::fs::read_to_string(home_dir.display().to_string())
        .expect("Could not read contents of configuration file");


    toml::from_str::<Data>(file_contents.as_str()).expect("Could not deserialzie the configuration file")
}

fn main() {
    let data = get_configuration();

    println!("{:#?}", data);
}
