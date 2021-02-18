#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket_contrib;

mod blockchain;
mod server;
mod sha;

fn main() {
    println!("Hello, Blockchain!");
    server::rocket().launch();
}
