use anyhow::Result;
use git2::Repository;
use serde::{Deserialize, Serialize};
use slack_hook::{PayloadBuilder, Slack};
use std::fs::remove_dir_all;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    slack_webhook_url: String,
    git_repo_url: String,
    tmp_prj_directory: String,
    aws_api_key: String,
    aws_secret: String,
    aws_default_region: String,
}

impl Project {
    pub fn process_plan(&self) -> Result<()> {
        let _ = Repository::clone(&self.git_repo_url, &self.tmp_prj_directory)?;

        let plan = self.generate_plan()?;

        self.post_plan_to_slack(plan);

        let _ = remove_dir_all(&self.tmp_prj_directory)?;

        Ok(())
    }

    pub fn generate_plan(&self) -> Result<String> {
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

    pub fn post_plan_to_slack(&self, plan: String) {
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
