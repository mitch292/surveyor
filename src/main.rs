mod project;

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::thread;
use structopt::StructOpt;

use crate::project::Project;

const DEFAULT_CONFIG_DIR: &str = ".config";
const DEFAULT_CONFIG_FILE_NAME: &str = ".surveyor.toml";

// TODO:
// - Should we have our own errors and give the users some more helpful messages?
// - Change the file structure?
// - Support more config file types?

fn main() -> Result<()> {
    let opts = Opt::from_args();

    let config_file_path = determine_config_path(opts.config)?;
    let config = Config::from_file(&config_file_path)?;

    let handle = thread::spawn(move || {
        for project in config.projects.iter() {
            let _ = project.process_plan();
            println!("\r\nPlan survey complete for {}\n\r\n", project.name);
        }
    });

    handle.join().unwrap();

    Ok(())
}

fn determine_config_path(explicit_path: Option<String>) -> Result<String> {
    if let Some(p) = explicit_path {
        return Ok(p);
    }

    if let Some(home) = home::home_dir() {
        let default_file = home.join(DEFAULT_CONFIG_DIR).join(DEFAULT_CONFIG_FILE_NAME);
        if let Some(dir) = default_file.to_str() {
            return Ok(String::from(dir));
        }
    }

    Err(Error::msg("No config file specified."))
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "surveyor",
    about = "Run terraform plan and send the output to slack"
)]
struct Opt {
    /// The location of your config file
    #[structopt(
        short,
        long,
        about = "Defaults to looking for a .surveyor.toml file inside of $HOME/.config/"
    )]
    config: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    projects: Vec<Project>,
}

impl Config {
    fn from_file(file_path: &str) -> Result<Self> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Self = toml::from_str(&contents)?;
        Ok(config)
    }
}
