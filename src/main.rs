mod args;

use args::{CreateUser, DeleteUser, ResultflixArgs};
use clap::{Command, Parser};

use crate::args::{EntityType, UserCommand, UserSubCommand};

pub fn list() {
    println!("List all todos");
}

pub fn create_user(user: &CreateUser) {
    println!("user {} with email {} is created", user.name, user.email);
}

pub fn delete_user(user: &DeleteUser) {
    println!("deliting user {}", user.user);
}
pub fn user_handler(v: &UserCommand) {
    match &v.command {
        UserSubCommand::Create(user) => create_user(user),
        UserSubCommand::Delete(user) => delete_user(user),
        _ => println!("Erro"),
    }
}

fn main() {
    let args = ResultflixArgs::parse();
    match args.entity_type {
        EntityType::User(v) => user_handler(&v),
        EntityType::List => list(),
        _ => println!("none"),
    }
}
