mod lexer;
#[cfg(test)]
mod lexer_test;
mod repl;
mod token;

use std::env;
use std::io;

#[macro_use]
extern crate lazy_static;

fn main() {
    let username = env::var("USER").unwrap_or_else(|_| "someone".to_string());
    println!(
        "Hello {}! This is the Monkey programming language!",
        username
    );
    println!("Feel free to type in commands");

    repl::start(&mut io::stdin().lock(), &mut io::stdout());
}
