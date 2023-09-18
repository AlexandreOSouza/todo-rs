mod args;

use args::{Commands, CreateTodo, DeleteTodo, ListTodo, MarkTodo, UnmarkTodo};
use clap::Parser;

use crate::args::CommandEntity;

pub fn list(v: &ListTodo) {
    println!("{:?}", v);
}

pub fn user_handler(v: &CreateTodo) {
    println!("{:?}", v);
}

pub fn mark_todo(v: &MarkTodo) {
    println!("{:?}", v);
}

pub fn unmark_todo(v: &UnmarkTodo) {
    println!("{:?}", v);
}

pub fn delete_todo(v: &DeleteTodo) {
    println!("{:?}", v);
}

fn main() {
    let args = Commands::parse();
    match args.command {
        CommandEntity::Create(v) => user_handler(&v),
        CommandEntity::List(val) => list(&val),
        CommandEntity::Mark(v) => mark_todo(&v),
        CommandEntity::Unmark(v) => unmark_todo(&v),
        CommandEntity::Delete(v) => delete_todo(&v),
    }
}
