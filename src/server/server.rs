use crate::Chain;
use rocket::*;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, sync::Mutex};
use rocket::config::{Config, Environment};
use rocket::response::status;

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

#[get("/nodes/resolve")]
fn resolve(state: State<Mutex<Chain>>) -> status::Accepted<String> {
    let mut chain = state.lock().unwrap();
    chain.resolve_conflicts();
    status::Accepted(Some("processing".to_string()))
}

#[derive(Serialize, Deserialize)]
struct NodeRequest {
    nodes: Vec<String>
}

#[post("/nodes/register", format = "json", data = "<body>")]
fn register_node(body: Json<NodeRequest>, state: State<Mutex<Chain>>) -> status::Accepted<String> {
    let mut chain = state.lock().unwrap();
    body.nodes.iter().for_each(| node | chain.register_node(node));
    status::Accepted(Some("nodes added".to_string()))
}

#[get("/nodes")]
fn nodes(state: State<Mutex<Chain>>) -> JsonValue {
    let chain = state.lock().unwrap();
    json!(chain.nodes())
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn rocket() -> rocket::Rocket {
    let chain = Mutex::new(Chain::new());
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(5000)
        .finalize().unwrap();

    rocket::custom(config)
        .manage(chain)
        .mount("/", routes![index, chain, mine, new_transaction, resolve, register_node, nodes])
        .register(catchers![not_found])
}
