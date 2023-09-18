mod args;

use args::{Commands, CreateTodo, ListTodo};
use clap::Parser;

use crate::args::CommandEntity;

pub fn list(v: &ListTodo) {
    println!("{:?}", v);
}

pub fn user_handler(v: &CreateTodo) {
    println!("{:?}", v);
}

fn main() {
    let args = Commands::parse();
    match args.command {
        CommandEntity::Create(v) => user_handler(&v),
        CommandEntity::List(val) => list(&val),
        _ => println!("none"),
    }
}
