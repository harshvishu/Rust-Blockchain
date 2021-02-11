use super::{Block, Transaction};
use crate::sha;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::Cursor;
use std::vec::Vec;
use uuid::Uuid;

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
            None => match self.chain.last_mut() {
                Some(last_block) => Chain::hash(last_block),
                None => "".to_string(),
            },
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
        self.current_transactions
            .push(Transaction::new(sender, recipient, amount));
        (self.current_transactions.len() - 1) as u64
    }

    pub fn hash(block: &Block) -> String {
        let json = serde_json::to_string(block).unwrap();
        let readable_string = Cursor::new(&json);
        sha::calc_sha_sum(readable_string).hash_string()
    }

    pub fn proof_of_work(last_proof: u64) -> u64 {
        let mut proof: u64 = 0;
        while Chain::is_valid_proof(last_proof, proof) {
            proof += 1;
        }
        proof
    }

    pub fn is_valid_proof(last_proof: u64, proof: u64) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let readable_guess = Cursor::new(&guess);
        let guess_hash = sha::calc_sha_sum(readable_guess);
        guess_hash.hash_string().ends_with(&guess)
    }

    pub fn is_valid_chain(chain: &Vec<Block>) -> bool {
        let chain_size = chain.len();
        let mut last_block = chain.first().unwrap();
        let mut current_index = 1;

        while current_index < chain_size {
            let block = &chain[current_index];
            let last_block_hash = Chain::hash(last_block);
            if block.previous_hash() != last_block_hash {
                return false;
            }

            if Chain::is_valid_proof(last_block.proof(), block.proof()) {
                return false;
            }
            last_block = block;
            current_index += 1;
        }
        true
    }

    pub fn register_node(&mut self, node: &str) {
        match Url::parse(node) {
            Ok(url) => {
                if let Some(host) = url.host_str() {
                    print!("new node : {} inserted.", host);
                    self.nodes.insert(host.to_string());
                }
            }
            Err(err) => {
                dbg!(err);
            }
        }
    }

    pub fn resolve_conflicts(&mut self) -> bool {
        let neighbours = &self.nodes;
        let mut new_chain: Vec<Block> = vec![];
        let mut max_length = self.chain.len();
        let mut chain_replaced = false;

        println!("... Resolving conflicts");

        for node in neighbours.iter() {
            let url = Url::parse(format!("http://{}:5000/chain", node).as_str()).unwrap();
            println!("... connecting to {}", url);
            match reqwest::blocking::get(url) {
                Ok(response) => {
                    #[derive(Deserialize, Debug)]
                    struct ResponseChain {
                        chain: Vec<Block>,
                        length: usize
                    }

                    match response.json::<ResponseChain>() {
                        Ok(data) => {
                            print!("{:?}", data);
                            if data.length > max_length && Chain::is_valid_chain(&data.chain) {
                                max_length = data.length;
                                new_chain = data.chain;
                                chain_replaced = true;

                                println!("... Chain was replaced");
                            }
                        }
                        Err(err) => {
                            dbg!(err);
                        }
                    }
                }
                Err(err) => {
                    dbg!(err);
                }
            }
        }

        if chain_replaced {
            self.chain.clear();
            self.chain.append(&mut new_chain);
        }

        println!("... Finished resolving conflicts");

        return true;
    }

    pub fn last_block(&mut self) -> Option<&mut Block> {
        self.chain.last_mut()
    }

    pub fn node_identifier() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn current_transactions(&self) -> &Vec<Transaction> {
        &self.current_transactions
    }

    pub fn count(&self) -> usize {
        self.chain.len()
    }

    pub fn nodes(&self) -> &HashSet<String> {
        &self.nodes
    }

    pub fn to_json(&self) -> String {
        let response = serde_json::json!({
             "chain" : self.chain,
             "length" : self.count()
        });
        response.to_string()
    }
}
