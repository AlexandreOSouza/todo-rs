use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Commands {
    #[clap(subcommand)]
    pub command: CommandEntity,
}

#[derive(Debug, Subcommand)]
pub enum CommandEntity {
    /// Create a new TODO item
    #[clap(short_flag = 'c')]
    Create(CreateTodo),
    /// List all TODOs based on params
    #[clap(short_flag = 'l')]
    List(ListTodo),
    /// Mark an TODO or mark all TODOs
    #[clap(short_flag = 'm')]
    Mark,
    /// Unmark an TODO or unmark all TODOs
    #[clap(short_flag = 'u')]
    Unmark,
}

#[derive(Debug, Args)]
pub struct CreateTodo {
    pub name: String,
}

#[derive(Debug, Args)]
pub struct ListTodo {
    #[arg(short = 'm')]
    pub marks: bool,
    #[arg(short = 'u')]
    pub unmarks: bool,
}
