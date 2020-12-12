#![allow(dead_code)]

use blockchain::{Block, Chain, Transaction};
use chrono::format::Numeric::Timestamp;
use chrono::Utc;

mod blockchain;
mod sha;

fn main() {
    println!("Hello, Blockchain!");
    let mut chain = Chain::new();

    chain.new_transaction("sender A".to_string(), "recipient B".to_string(), 50);
    chain.new_transaction("sender C".to_string(), "recipient D".to_string(), 150);

    match chain.last_block() {
        Some(last_block) => {
            let previous_hash = Chain::hash(last_block);
            let new_block = chain.new_block(Some(previous_hash), 200);
        },
        None => {}
    };
    

    // chain.new_block(None, proof)
    // let block = Block::new(&chain, vec![], 0, previous_hash.to_string());

    dbg!(chain);
    // block.transactions.push(Transaction {
    //     sender: "Harsh".to_string(),
    //     recipient: "Ajay".to_string(),
    //     amount: 50,
    // });

    // block.transactions.push(Transaction {
    //     sender: "Harsh".to_string(),
    //     recipient: "Just Wravel".to_string(),
    //     amount: 5000,
    // });clea

    // chain

    // chain.chain().push(block);

    // dbg!(chain);
}
