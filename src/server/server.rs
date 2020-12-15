use crate::Chain;
use rocket::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[get("/")]
fn index(state: State<Mutex<Chain>>) -> String {
    let chain = state.lock().unwrap();
    chain.to_json()
}

#[get("/chain")]
fn chain(state: State<Mutex<Chain>>) -> String {
    let chain = state.lock().unwrap();
    chain.to_json()
}

#[get("/mine")]
fn mine(state: State<Mutex<Chain>>) -> String {
    let mut chain = state.lock().unwrap();

    match chain.last_block() {
        Some(last_block) => {
            let previous_hash = Chain::hash(last_block);
            chain.new_block(Some(previous_hash), 200);
            chain.to_json()
        }
        None => "502 Internal Server error".to_string(),
    }
}

#[derive(Serialize, Deserialize)]
struct TransactionRequest {
    sender: String,
    recipient: String,
    amount: u64,
}

#[post("/transactions/new", format = "json", data = "<body>")]
fn new_transaction(body: Json<TransactionRequest>, state: State<Mutex<Chain>>) -> String {
    let mut chain = state.lock().unwrap();
    let index = chain.new_transaction(body.0.sender, body.0.recipient, body.0.amount);
    chain.current_transactions().get(index as usize).unwrap().to_json()
}

pub fn rocket() -> rocket::Rocket {
    let chain = Mutex::new(Chain::new());
    rocket::ignite()
        .manage(chain)
        .mount("/", routes![index, chain, mine, new_transaction])
}
