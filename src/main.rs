pub mod models;

use std::error::Error;
use std::fmt;
use std::process::exit;

use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[allow(dead_code, unused_attributes, unused_variables)]

lazy_static! {
    static ref url: String = "https://copr.fedorainfracloud.org/api_3".to_string();
}

#[derive(Debug)]
struct ArgumentError(String);

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for ArgumentError {}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Build {
        #[arg(short, long)]
        build_id: Option<u64>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename(deserialize = "copr-cli"))]
    coprcli: Option<CoprCli>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CoprCli {
    pub login: Option<String>,
    pub username: Option<String>,
    pub token: Option<String>,
    pub copr_url: Option<String>,
}

fn get_configuration() -> Data {
    let args = Args::parse();

    let mut home_dir = simple_home_dir::home_dir().expect("Could not find home path!");
    home_dir.push(".config");
    home_dir.push("copr");

    if !home_dir.exists() {
        panic!("The configuration file does not exist!");
    }

    let file_contents = std::fs::read_to_string(home_dir.display().to_string())
        .expect("Could not read contents of configuration file");

    toml::from_str::<Data>(file_contents.as_str())
        .expect("Could not deserialzie the configuration file")
}

fn fetch_build(build_id: Option<u64>) -> Result<(), Box<dyn Error>> {
    let mut endpoint = url.to_string();
    endpoint.push_str("/build/");

    if let Some(id) = build_id {
        endpoint.push_str(id.to_string().as_str());
        let resp = reqwest::blocking::get(endpoint)?.text()?;
        // println!("{:#?}", resp);
        let build: models::build::Build = serde_json::from_str(resp.as_str())?;
        println!("{:#?}", build);
        return Ok(());
    }

    Err(Box::new(ArgumentError(String::from("Missing a build id!"))))
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_configuration();

    // println!("{:#?}", data);
    // println!("{}", url.to_string());

    let args = Args::parse();

    let command = match args.command {
        Some(c) => c,
        None => {
            println!("No sub command provided. Please look at the help options.");
            exit(0);
        }
    };

    match command {
        Commands::Build { build_id } => {
            if let Err(e) = fetch_build(build_id) {
                println!("{}", e);
            }
        }
        _ => {
            println!("Unrecognized command");
            exit(0);
        }
    }

    Ok(())
}
