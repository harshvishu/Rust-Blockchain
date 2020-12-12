use std::collections::HashSet;
use std::vec::Vec;
use std::io::Read;

use super::{Block, Transaction};
use crate::sha;

#[derive(Debug)]
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

    pub fn new_block(&mut self, previous_hash: Option<String>, proof: i64) {
        let previous_hash: String = match previous_hash {
            Some(previous_hash) => previous_hash.to_string(),
            None => {
                match self.chain.last_mut() {
                    Some(last_block) => {
                        Chain::hash(last_block)
                    },
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
        let transaction = Transaction::new(sender, recipient, amount);
        // let new_index: u64 = (self.count() + 1) as u64;
        self.current_transactions.push(transaction);

        if let Some(last_block) = self.chain.last() {
            return last_block.index();
            // last_block.update_index(new_index);
        }
        return 0;
    }

    pub fn hash(block: &mut Block) -> String {
        // let mut buffer: Vec<u8> = Vec::new();
        // let result = block.read(buffer.as_mut());
        sha::calc_sha_sum(block).hash_string()
    }

    pub fn last_block(&mut self) -> Option<&mut Block> {
        self.chain.last_mut()
    }

    pub fn count(&self) -> usize {
        self.chain.len()
    }
}
