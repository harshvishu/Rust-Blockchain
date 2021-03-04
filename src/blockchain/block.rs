use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str;
use std::vec::Vec;
use super::{Chain, Transaction};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    transactions: Vec<Transaction>,
    proof: u64,
    previous_hash: String,
}

impl Block {
    pub fn new(
        chain: &Chain,
        transactions: Vec<Transaction>,
        proof: u64,
        previous_hash: String,
    ) -> Self {
        Block {
            index: (chain.count() + 1) as u64,
            timestamp: Utc::now(),
            transactions,
            proof,
            previous_hash,
        }
    }

    pub fn previous_hash(&self) -> String {
        format!("{}", self.previous_hash)
    }
    
    pub fn proof(&self) -> u64 {
        self.proof
    }
}
