use anyhow::Result;
use clap::{value_parser, Parser};
use dotenv_codegen::dotenv;
use rust_psysswd::cli::{CliArgs, CliCommands};
use rust_psysswd::{config, persist, read_username_and_password};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CliArgs::parse();
    let vault_conf = config::VaultConfig::load_yaml(cli.conf.as_ref()).await?;
    let username_and_password =
        read_username_and_password(cli.username, cli.password, &vault_conf)?;

    // println!("dotenv! PORT: {:?}", dotenv!("PORT"));
    // println!("home: {:?}, {:?}", std::env::var("HOME"), dirs::home_dir());
    // for (key, val) in env::vars_os() {
    //     println!("key {:?}, val {:?}", key, val);
    // }

    // println!(
    //     "cli.conf:{:?}, config:{:?}, username_and_password:{:?}",
    //     &cli.conf, &vault_conf, username_and_password
    // );

    let persist_data = vault_conf
        .persist
        .map(|persist_conf| persist_conf.data_path)
        .unwrap_or("~/.pvlt/file.data".to_string());
    persist::init_db(&persist_data).await?;

    match cli.command {
        CliCommands::List(list_args) => {
            println!("list: {:?}", list_args);
        }
        CliCommands::Find(find_args) => {
            println!(
                "search_key:{}, plain:{}",
                find_args.search_key, find_args.plain
            );

            let result = persist::dao::query_record(
                username_and_password.0,
                username_and_password.1,
                find_args.search_key,
            )
            .await?;
            println!("search result: {:?}", result);
        }
        CliCommands::Add(add_args) => {
            println!("add: {:?}", add_args);
        }
    }

    Ok(())
}
