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
    Mark(MarkTodo),
    /// Unmark an TODO or unmark all TODOs
    #[clap(short_flag = 'u')]
    Unmark(UnmarkTodo),
    /// Delete a TODO
    #[clap(short_flag = 'd')]
    Delete(DeleteTodo),
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

#[derive(Debug, Args)]
pub struct MarkTodo {
    #[arg(short = 'n')]
    pub name: Option<String>,
}

#[derive(Debug, Args)]
pub struct UnmarkTodo {
    #[arg(short = 'n')]
    pub name: Option<String>,
}

#[derive(Debug, Args)]
pub struct DeleteTodo {
    #[arg(short = 'n')]
    pub name: Option<String>,
}
