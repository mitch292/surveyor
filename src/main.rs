use anyhow::Result;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();

    println!("{:?}", opt);
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "surveyor",
    about = "Run terraform plan and send the output to slack"
)]
struct Opt {
    /// The project to survey
    #[structopt(short, long)]
    project: String,

    /// The location of your config file
    #[structopt(short, long, default_value = "~/.surveyor.toml")]
    config: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    slack_webhook_url: String, // TODO: Can this be a url type?
    git_repo_url: String,
    aws_api_key: String,
    aws_secret: String,
    aws_default_region: String,
}
