#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket_contrib;

use blockchain::{Chain, Block};

mod blockchain;
mod server;
mod sha;

fn main() {
    println!("Hello, Blockchain!");
    let mut chain = Chain::new();

    chain.new_transaction("0".to_string(), Chain::node_identifier(), 1);
    chain.new_transaction("Harsh".to_string(), "Batman".to_string(), 10_000_00);

    match chain.last_block() {
        Some(last_block) => {
            let previous_hash = Chain::hash(last_block);
            chain.new_block(Some(previous_hash), 200);
        }
        None => {}
    };

    chain.new_transaction("Harsh".to_string(), "Superman".to_string(), 20);
    chain.new_transaction("Harsh".to_string(), "Spider man".to_string(), 70);
    chain.new_transaction("Batman".to_string(), "Spider man".to_string(), 20000);

    match chain.last_block() {
        Some(last_block) => {
            let previous_hash = Chain::hash(last_block);
            chain.new_block(Some(previous_hash), 200);
        }
        None => {}
    };

    chain.new_transaction("Popoye".to_string(), "Brutus".to_string(), 500);
    server::rocket().launch();
}
