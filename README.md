## What is Surveyor?

Surveyor is a cli tool that you can use to keep track of the state of your infrastructure compared to your terraform configuration. It'll clone your git repo, run terraform plan, output that to a slack channel.

## What's required to use Surveyor?

Surveyor is very much in MVP / single use case stage of development. To run today, you'll need a couple dependencies resolved.

- Your own slack application with access to your workspace
- A machine with git installed and access to your git repo - ONLY GIT OVER HTTPS IS SUPPORTED
- Valid AWS credentials to run `terraform plan` - ONLY AWS IS SUPPORTED

## How do I use Surveyor?

You configure a `.surveyor.toml` file that details the projects you want generate slack updates for. By default surveyor will look for this file in $HOME/.config/.surveyor, but you can specify the location via a command line argument

To output a plan to slack for a single project you can use:

```
surveyor -p=my_infra
```

or

```
surveyor --project=my_infra
```

To output a plan to slack for all your configured projects you can use:

```
surveyor
```

## What should my .surveyor.toml file look like?

The `.surveyor.toml` file should be very simple. Below is a sample file structure.

```toml
[[projects]]
name = "my_infra"
slack_webhook_url = "https://hooks.slack.com/services/YOUR_WEBHOOK_URL"
git_repo_url = "https://github.com/your_user/your_repo.git"
tmp_prj_directory = "FULL_PATH_OF_DIR_TO_CLONE_REPO_TEMPORARILY"
aws_api_key = "YOUR_AWS_API_KEY"
aws_secret = "YOUR_AWS_API_KEY_SECRET"
aws_default_region = "YOUR_AWS_DEFAULT_REGION"

[[projects]]
name = "another_project"
slack_webhook_url = "https://hooks.slack.com/services/YOUR_WEBHOOK_URL"
git_repo_url = "https://github.com/your_user/your_repo.git"
tmp_prj_directory = "FULL_PATH_OF_DIR_TO_CLONE_REPO_TEMPORARILY"
aws_api_key = "YOUR_AWS_API_KEY"
aws_secret = "YOUR_AWS_API_KEY_SECRET"
aws_default_region = "YOUR_AWS_DEFAULT_REGION"
```
