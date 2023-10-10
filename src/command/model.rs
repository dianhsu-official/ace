use clap::{Args, Subcommand};

use crate::constants::ProgramLanguage;
#[derive(Subcommand)]
pub enum Commands {
    /// Manage account for ace, such as add, remove, list
    Account(AccountArgs),
    /// Manage config for ace, such as set, get, remove
    Config(ConfigArgs),
    /// Manage language for ace, such as set, list
    Lang(LanguageArgs),
    /// Parse the problem
    Parse(ParseArgs),
    /// Generate template
    Gen(GenerateArgs),
    /// Submit the code
    Submit(SubmitArgs),

    /// Test the code
    Test(TestArgs),
}

#[derive(Subcommand)]
pub enum AccountOptions {
    Add,
    List,
    SetDefault,
    Update,
    Delete,
}
#[derive(Args)]
pub struct AccountArgs {
    #[command(subcommand)]
    pub options: AccountOptions,
    #[arg(short, long)]
    pub platform: Option<String>,
}

#[derive(Subcommand)]
pub enum ConfigOptions {
    /// Get config value
    Get,
    /// Add config name and value
    Add,
    /// List all config
    List,
    /// Set config value
    Update,
    /// Remove config name and value
    Delete,
}
#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub options: ConfigOptions,
}

#[derive(Subcommand)]
pub enum LanguageOptions {
    /// List all language config
    List,
    /// Set language config
    Add,
    /// Delete language config
    Delete,
}

#[derive(Args)]
pub struct LanguageArgs {
    #[command(subcommand)]
    pub options: LanguageOptions,
}

#[derive(Args)]
pub struct ParseArgs {
    pub platform: String,
    pub contest_identifier: String,
}

#[derive(Args)]
pub struct GenerateArgs {
    pub language: Option<ProgramLanguage>,
}

#[derive(Args)]
pub struct SubmitArgs {
    pub filename: Option<String>,
}

#[derive(Args)]
pub struct TestArgs {
    pub filename: Option<String>,
}
