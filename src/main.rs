use std::env;
use std::fs;

use anyhow::Context;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

const AWS_ACCOUNT_ID_ENV: &str = "AWS_ACCOUNT_ID";
const BASE_CONFIG_DEFAULT_PATH: &str = "$HOME/.config/";
const ACCOUNTS_FILE: &str = "aws_cli_account_info/accounts.yaml";
const ACCOUNTS_KEY: &str = "accounts";

fn main() -> anyhow::Result<()> {
    let config_path = match env::var("XDG_CONFIG_HOME") {
        Ok(path) => format!("{path}{ACCOUNTS_FILE}"),
        Err(_) => format!("{BASE_CONFIG_DEFAULT_PATH}{ACCOUNTS_FILE}"),
    };

    let config_path_expanded = shellexpand::env(&config_path)?.into_owned();
    let yaml_accounts = match fs::read_to_string(&config_path_expanded) {
        Ok(raw) => Some(YamlLoader::load_from_str(&raw)?),
        Err(_e) => None,
    };

    let accounts = match &yaml_accounts {
        Some(accounts) => Some(
            accounts
                .first()
                .context("Accounts yaml is empty")?
                .as_hash()
                .context("Accounts yaml cannot be parsed as HashMap")?
                .get(&Yaml::from_str(ACCOUNTS_KEY))
                .context("`accounts` keyword not found in Accounts yaml")?
                .as_hash()
                .context("Accounts cannot be parsed as HashMap")?,
        ),
        None => None,
    };

    let mut aws_account_name = match env::var(AWS_ACCOUNT_ID_ENV) {
        Ok(account_id) => account_id,
        Err(_) => "".to_string(),
    };

    if let Some(accounts) = accounts {
        aws_account_name = match &accounts.get(&Yaml::from_str(&aws_account_name)) {
            Some(alternative) => alternative
                .as_str()
                .context("Cannot parse alternative name to str")?
                .to_string(),
            None => aws_account_name,
        };
    }

    if aws_account_name == "" {
        return Ok(());
    }
    println!("%{{%}}(AWS: {}%{{%}}) ", aws_account_name);

    return Ok(());
}
