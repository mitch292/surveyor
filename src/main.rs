use anyhow::{Error, Result};
use git2::Repository;
use serde::{Deserialize, Serialize};
use slack_hook::{PayloadBuilder, Slack};
use std::fs::{remove_dir_all, File};
use std::io::prelude::*;
use std::process::Command;
use structopt::StructOpt;

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

    for project in config.projects.iter() {
        let _ = project.process_plan();
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

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    slack_webhook_url: String,
    git_repo_url: String,
    tmp_prj_directory: String,
    aws_api_key: String,
    aws_secret: String,
    aws_default_region: String,
}

impl Project {
    fn process_plan(&self) -> Result<()> {
        let _ = Repository::clone(&self.git_repo_url, &self.tmp_prj_directory)?;

        let plan = self.generate_plan()?;

        self.post_plan_to_slack(plan);

        let _ = remove_dir_all(&self.tmp_prj_directory)?;

        Ok(())
    }

    fn generate_plan(&self) -> Result<String> {
        let mut init = Command::new("terraform");
        init.arg("init")
            .current_dir(&self.tmp_prj_directory)
            .status()?;

        let mut plan = Command::new("terraform");
        let plan_output = plan
            .arg("plan")
            .arg("-no-color")
            .current_dir(&self.tmp_prj_directory)
            .env("AWS_ACCESS_KEY_ID", &self.aws_api_key)
            .env("AWS_SECRET_ACCESS_KEY", &self.aws_secret)
            .env("AWS_DEFAULT_REGION", &self.aws_default_region)
            .output()?;

        let plan = std::str::from_utf8(&plan_output.stdout)?;

        Ok(String::from(plan))
    }

    fn post_plan_to_slack(&self, plan: String) {
        let slack = Slack::new(self.slack_webhook_url.as_str()).unwrap();
        let p = PayloadBuilder::new()
            .text(remove_refresh_message(plan))
            .build()
            .unwrap();

        let _res = slack.send(&p);
    }
}

/// Removes unnecessary output from the terraform message
/// If terraform ever changes the output of their message, this will break.
/// TODO: Add "supported" cli versions that have this output, or remove this logic
fn remove_refresh_message(s: String) -> String {
    let split: Vec<&str> = s
        .split("------------------------------------------------------------------------")
        .collect();

    String::from(split[split.len() - 2])
}
