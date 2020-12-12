use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u64,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Self {
        Transaction {
            sender,
            recipient,
            amount,
        }
    }
}
