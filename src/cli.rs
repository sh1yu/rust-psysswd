use clap::{Parser, Subcommand};

/// A password vault for your password security (rust).
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliCommands,

    /// config file
    #[arg(short, long)]
    pub conf: Option<String>,

    /// give your username
    #[arg(short, long)]
    pub username: Option<String>,

    /// give your master password
    #[arg(short, long)]
    pub password: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum CliCommands {
    /// list account info for given username
    #[command(arg_required_else_help = false)]
    List(ListArgs),

    /// find given account info
    #[command(arg_required_else_help = true)]
    Find(FindArgs),

    /// add a new account for given username
    Add(AddArgs),
}

#[derive(Parser, Debug)]
pub struct ListArgs {
    /// if true, print password in plain text
    #[arg(short = 'P', long)]
    pub plain: bool,
}

#[derive(Parser, Debug)]
pub struct FindArgs {
    /// search key
    pub search_key: String,

    /// if true, print password in plain text
    #[arg(short = 'P', long)]
    pub plain: bool,
}

#[derive(Parser, Debug)]
pub struct AddArgs {
    /// account name to add
    pub account_name: String,

    /// account user to add
    pub account_user: String,

    /// extra message for this account, optional
    pub extra_message: Option<String>,

    /// if true, generate a random password
    #[arg(short, long)]
    pub gen_pass: bool,
}
