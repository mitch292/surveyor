mod project;

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;
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
    let mut stdout = io::stdout();
    let mut project_found = false;

    let config_file_path = determine_config_path(opts.config)?;
    let config = Config::from_file(&config_file_path)?;

    for project in config.projects.iter() {
        if opts.project.is_some() {
            if opts.project.as_ref().unwrap() == &project.name {
                project_found = true;
                let _ = project.process_plan();
                write!(
                    stdout,
                    "\r\nPlan survey complete for {}\n\r\n",
                    project.name
                )?;
            }
        } else {
            project_found = true;
            let _ = project.process_plan();
            write!(
                stdout,
                "\r\nPlan survey complete for {}\n\r\n",
                project.name
            )?;
        }
    }

    if !project_found {
        write!(
            stdout,
            "\r\nNo valid projects found in the config file!\n\r\n"
        )?;
    }

    Ok(())
}

fn determine_config_path(explicit_path: Option<String>) -> Result<String> {
    if let Some(p) = explicit_path {
        return Ok(String::from(p));
    }

    if let Some(home) = home::home_dir() {
        let default_file = home.join(DEFAULT_CONFIG_DIR).join(DEFAULT_CONFIG_FILE_NAME);
        if let Some(dir) = default_file.to_str() {
            return Ok(String::from(dir));
        }
    }

    return Err(Error::msg("No config file specified."));
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "surveyor",
    about = "Run terraform plan and send the output to slack"
)]
struct Opt {
    /// The project to survey
    #[structopt(short, long)]
    project: Option<String>,

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