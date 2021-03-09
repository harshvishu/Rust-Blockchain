use super::{Block, Transaction};
use crate::sha;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet};
use std::io::Cursor;
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    nodes: HashSet<String>,
}

impl Chain {
    pub fn new() -> Self {
        let mut chain = Chain {
            chain: vec![],
            current_transactions: vec![],
            nodes: HashSet::new(),
        };
        chain.new_block(Some("1".to_string()), 100);
        return chain;
    }

    pub fn new_block(&mut self, previous_hash: Option<String>, proof: u64) -> (u64, Vec<Transaction>) {
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
            previous_hash,
        );

        let return_value =  (block.index(), block.transactions().clone());

        self.chain.push(block);
        self.current_transactions.clear();

        return_value
    }

    /**
 Creates a new transaction to go into the next mined Block

 - Parameter sender: Address of the Sender
 - Parameter recipient: Address of the Recipient
 - Parameter amount: Amount
 - returns: The index of the newly created transaction
  */
    pub fn new_transaction(&mut self, sender: String, recipient: String, amount: u64) -> u64 {
        self.current_transactions
            .push(Transaction::new(sender, recipient, amount));
        (self.current_transactions.len() - 1) as u64
    }

    /**
 Creates a SHA-256 hash of a Block

 - Parameter block: <dict> Block
 - returns: String

 */
    pub fn hash(block: &Block) -> String {
        let json = serde_json::to_string(block).unwrap();
        let readable_string = Cursor::new(&json);
        sha::calc_sha_sum(readable_string).hash_string()
    }

    /**
 Simple Proof of Work Algorithm:

 - Find a number p' such that hash(pp') contains leading 4 zeroes, where p is the previous p'
 - p is the previous proof, and p' is the new proof
 - Parameter: last_proof: Int64
 - returns: Int64
 */
    pub fn proof_of_work(last_proof: u64) -> u64 {
        let mut proof: u64 = 0;
        while !Chain::is_valid_proof(last_proof, proof) {
            proof += 1;
        }
        proof
    }

    /**
 Validates the Proof: Does hash(last_proof, proof) contain 4 leading zeroes?

 - Parameter last_proof: <int> Previous Proof
 - Parameter proof: <int> Current Proof
 - returns: True if correct, False if not.
 */
    pub fn is_valid_proof(last_proof: u64, proof: u64) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let readable_guess = Cursor::new(&guess);
        let guess_hash = sha::calc_sha_sum(readable_guess);
        guess_hash.hash_string().ends_with("0000")
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

        chain_replaced
    }

    pub fn last_block(&mut self) -> Option<&mut Block> {
        self.chain.last_mut()
    }

    pub fn count(&self) -> usize {
        self.chain.len()
    }

    pub fn nodes(&self) -> &HashSet<String> {
        &self.nodes
    }

    pub fn current_transactions(&self) -> &Vec<Transaction> {
        &self.current_transactions
    }

    pub fn to_json(&self) -> serde_json::Value {
        let response = serde_json::json!(self.chain);
        response
    }
}
