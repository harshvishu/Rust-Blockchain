#![allow(dead_code)]

use blockchain::{Block, Chain, Transaction};
use chrono::format::Numeric::Timestamp;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Result;

mod blockchain;
mod sha;

fn main() {
    println!("Hello, Blockchain!");
    let mut chain = Chain::new();

    let last_block = chain.last_block();
    let last_proof = last_block.and_then(|e| Some(e.proof()));
    let proof = match last_proof {
        Some(last_proof) => Chain::proof_of_work(last_proof),
        None => 0
    };

    chain.new_transaction("0".to_string(), Chain::node_identifier(), 1);
    chain.new_transaction("Harsh".to_string(), "Batman".to_string(), 10_000_00);

    // chain.new_transaction("sender C".to_string(), "recipient D".to_string(), 150);

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

    // chain.new_block(None, proof)
    // let block = Block::new(&chain, vec![], 0, previous_hash.to_string());

    let chain_in_json = serde_json::to_string_pretty(&chain).unwrap_or("".to_string());
    println!("{}", chain_in_json);

    dbg!(Chain::is_valid_chain(chain.chain()));

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
