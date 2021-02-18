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

    #[serde(skip)]
    cursor: usize,
    #[serde(skip)]
    encoded: Option<Vec<u8>>,
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
            cursor: 0,
            encoded: None,
        }
    }

    pub fn update_index(&mut self, index: u64) {
        self.index = index;
    }

    pub fn index(&self) -> u64 {
        self.index
    }

    pub fn previous_hash(&self) -> String {
        format!("{}", self.previous_hash)
    }
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
    pub fn proof(&self) -> u64 {
        self.proof
    }
    
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or("".to_string())
    }
}
