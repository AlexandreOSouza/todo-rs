use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ResultflixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// User subcommand
    User(UserCommand),

    /// List aoo TODOs
    List,
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubCommand {
    /// create user
    Create(CreateUser),
    /// Delete user
    Delete(DeleteUser),
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// Name of the user
    pub name: String,
    /// Email of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct DeleteUser {
    /// Name of the user
    pub user: String,
}
