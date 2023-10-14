use anyhow::{anyhow, Result};
use rpassword::prompt_password;

pub mod cli;
pub mod config;
pub mod persist;

pub fn read_username_and_password(
    username_args: Option<String>,
    password_args: Option<String>,
    vault_conf: &config::VaultConfig,
) -> Result<(String, String)> {
    let username = username_args.unwrap_or(vault_conf.user.default_user_name.clone());
    if username == "" {
        return Err(anyhow!(
            "please give your master user name with -u or 'user.defaultUserName' in config file."
        ));
    }
    let password = match password_args {
        Some(password) => password,
        None => prompt_password("please input your master password:")?,
    };

    Ok((username, password))
}
