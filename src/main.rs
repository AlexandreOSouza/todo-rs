mod args;

use args::Commands;
use clap::Parser;
use todo_rs::Todo;

use crate::args::CommandEntity;

fn main() {
    let mut todo = Todo::new().expect("Error creating the todo");

    let args = Commands::parse();
    match args.command {
        CommandEntity::Create(new_todo) => todo.create(new_todo.name),
        CommandEntity::List(val) => todo.list(val.marks, val.unmarks),
        CommandEntity::Mark(v) => todo.mark(v.name),
        CommandEntity::Unmark(v) => todo.unmark(v.name),
        CommandEntity::Delete(v) => todo.delete(v.name),
    }
}
