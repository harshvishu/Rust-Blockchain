use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::io::{Error, ErrorKind};
use std::str;
use std::vec::Vec;
use serde_json::{Result as JsonResult};
use super::{Chain, Transaction, transaction};

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
        // let previous_hash = previous_hash.unwrap_or("");
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

impl Read for Block {
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, Error> {
        // let bytes: &[u8] = unsafe { any_as_u8_slice(&self) };
        // buf.clone_from_slice(bytes);
        // dbg!(bytes.len());
        if self.encoded == None {
            self.encoded = Some(bincode::serialize(&self).unwrap());
        }

        match &self.encoded {
            Some(encoded) => {
                if buf.len() == 0 {
                    self.cursor = 0;
                    self.encoded = None;

                    println!(">>>>>>>>> pass 1");

                    return Ok(0); // buffer empty
                }

                if self.cursor >= encoded.len() {
                    self.cursor = 0;
                    self.encoded = None;

                    println!(">>>>>>>>> pass 2");

                    return Ok(0); // reached end
                }

                let mut remaining_bytes = encoded.len() - self.cursor;
                if remaining_bytes > buf.len() {
                    println!(">>>>>>>>> pass 5");

                    remaining_bytes = buf.len() // we have more data then the buffer size, we take only the buffer size
                } else {
                    println!(">>>>>>>>> pass 6");
                }
                println!("remaining bytes {}", remaining_bytes);

                buf[..remaining_bytes].clone_from_slice(&encoded[..remaining_bytes]);
                self.cursor += remaining_bytes;

                println!(">>>>>>>>> pass 3");

                Ok(remaining_bytes)
            }
            None => {
                self.cursor = 0;
                self.encoded = None;

                println!(">>>>>>>>> pass 4");

                Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    "failed to fill whole buffer",
                ))
            }
        }

        // self.encoded = bincode::serialize(&self).unwrap();
        // let size = buf.len();
        // // buf[..size].clone_from_slice(&encoded[..size]);
        // Ok(0)
        // Err(Error::new(ErrorKind::UnexpectedEof, "failed to fill whole buffer"))
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let mut encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        buf.clone_from(encoded.as_mut());
        // Ok(encoded.len())
        Err(Error::new(
            ErrorKind::UnexpectedEof,
            "failed to fill whole buffer",
        ))
    }
}
