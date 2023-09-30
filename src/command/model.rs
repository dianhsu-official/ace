use clap::{Args, Subcommand};
#[derive(Subcommand)]
pub enum Commands {
    /// Manage account for ace, such as add, remove, list
    Account(AccountArgs),
    /// Manage config for ace, such as set, get, remove
    Config(ConfigArgs),

    Lang(LanguageArgs),
}

#[derive(Subcommand)]
pub enum AccountOptions {
    Add,
    List,
    SetDefault,
    Update,
    Remove,
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
    Set,
    /// Remove config name and value
    Remove,
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
    Set,
}

#[derive(Args)]
pub struct LanguageArgs {
    #[command(subcommand)]
    pub options: LanguageOptions,
}
