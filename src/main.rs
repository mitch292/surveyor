use anyhow::{Error, Result};
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::fs::{remove_dir_all, File};
use std::io::prelude::*;
use structopt::StructOpt;

const DEFAULT_CONFIG_DIR: &str = ".config";
const DEFAULT_CONFIG_FILE_NAME: &str = ".surveyor.toml";

fn main() -> Result<()> {
    let opts = Opt::from_args();

    let config_path = determine_config_path(opts.config)?;
    let projects = get_projects_from_config_file(config_path)?;

    for project in projects.iter() {
        let _ = process_project_plan(project);
    }
    Ok(())
}

fn process_project_plan(prj: &Project) -> Result<()> {
    let _ = Repository::clone(&prj.git_repo_url, &prj.tmp_prj_directory)?;
    remove_dir_all(&prj.tmp_prj_directory)?;
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

fn get_projects_from_config_file(file_path: String) -> Result<Vec<Project>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;

    Ok(config.projects)
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

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    slack_webhook_url: String, // TODO: Can this be a url type?
    git_repo_url: String,
    tmp_prj_directory: String,
    aws_api_key: String,
    aws_secret: String,
    aws_default_region: String,
}
