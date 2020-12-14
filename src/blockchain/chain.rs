use std::collections::HashSet;
use std::vec::Vec;
use std::io::{Read, Cursor};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use super::{Block, Transaction};
use crate::sha;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    nodes: HashSet<String>,
}

// pub trait Chain {
//     fn get_chain(&mut self) -> &mut Vec<Block>;
//     // fn get_current_transactions(&self) -> &Vec<Transaction>;
//     // fn get_nodes() -> HashSet<String>;
//     // fn new_block(previous_hash: String, proof: i64) -> Block;
//     // fn new_transaction(sender: String, recipient: String, amount: i64) -> i64;
//     // fn get_last_block() -> Block;
//     // fn hash(block: Block) -> String;
//     // fn proof_of_work(last_proof: i64) -> i64;
//     // fn valid_proof(last_proof: i64, proof: i64) -> bool;
//     // fn register_node(address: String) -> bool;
//     // fn is_valid_chain(chain: [Block]) -> bool;
//     // fn resolve_conflicts() -> bool;
// }

impl Chain {
    pub(crate) fn new() -> Self {
        let mut chain = Chain {
            chain: vec![],
            current_transactions: vec![],
            nodes: HashSet::new(),
        };
        chain.new_block(Some("1".to_string()), 100);
        return chain;
    }

    pub fn chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn new_block(&mut self, previous_hash: Option<String>, proof: u64) {
        let previous_hash: String = match previous_hash {
            Some(previous_hash) => previous_hash.to_string(),
            None => {
                match self.chain.last_mut() {
                    Some(last_block) => {
                        Chain::hash(last_block)
                    }
                    None => {
                        "".to_string()
                    }
                }
            }
        };

        let block = Block::new(
            &self,
            self.current_transactions.to_vec(),
            proof,
            previous_hash.to_string(),
        );
        self.chain.push(block);
        self.current_transactions.clear();
    }

    pub fn new_transaction(&mut self, sender: String, recipient: String, amount: u64) -> u64 {
        // let transaction = Transaction::new(sender, recipient, amount);
        // let new_index: u64 = (self.count() + 1) as u64;
        self.current_transactions.push(Transaction::new(sender, recipient, amount));
        match self.chain.last() {
            Some(last_block) => last_block.index(),
            None => 0
        }
    }

    pub fn hash(block: &mut Block) -> String {
        sha::calc_sha_sum(block).hash_string()
    }

    pub fn proof_of_work(last_proof: u64) -> u64 {
        let mut proof: u64 = 0;
        while Chain::valid_proof(last_proof, proof) {
            proof += 1;
        }
        proof
    }

    pub fn valid_proof(last_proof: u64, proof: u64) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let readable_guess = Cursor::new(&guess);
        let guess_hash = sha::calc_sha_sum(readable_guess);
        guess_hash.hash_string().ends_with(&guess)
    }

    pub fn is_valid_chain(chain: Vec<Block>) -> bool {
        false
    }

    pub fn last_block(&mut self) -> Option<&mut Block> {
        self.chain.last_mut()
    }

    pub fn node_identifier() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn count(&self) -> usize {
        self.chain.len()
    }
}