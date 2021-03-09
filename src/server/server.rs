use crate::blockchain::Chain;
use rocket::*;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use std::{sync::Mutex};
use rocket::config::{Config, Environment};
use rocket::response::{Redirect, Response};
use rocket::http::{Status, ContentType};
use std::io::Cursor;

#[get("/")]
fn index(_state: State<Mutex<Chain>>) -> Redirect {
    Redirect::to(uri!(chain))
}

#[get("/chain")]
fn chain(state: State<Mutex<Chain>>) -> Response {
    let chain = state.lock().unwrap();
    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(chain.to_json().to_string()))
        .finalize();
    response
}

#[get("/mine")]
fn mine(state: State<Mutex<Chain>>) -> Response {
    let mut chain = state.lock().unwrap();
    let mut response_builder = Response::build();
    let result = match chain.last_block() {
        Some(last_block) => {
            let last_proof = last_block.proof();
            let proof = Chain::proof_of_work(last_proof);
            let previous_hash = Chain::hash(last_block);
            let block = chain.new_block(Some(previous_hash.to_owned()), proof);

            let result = json!({
             "message" : "New block forged",
             "proof" : proof,
             "previous_hash": previous_hash,
             "index" : block.0,
             "transactions" : block.1,
        }).to_string();
            response_builder
                .status(Status::Ok)
                .header(ContentType::JSON)
                .sized_body(Cursor::new(result))
        }
        None => {
            response_builder
                .status(Status::InternalServerError)
                .header(ContentType::HTML)
                .sized_body(Cursor::new("Unable to mine a new block"))
        }
    };
    result.finalize()
}

#[derive(Serialize, Deserialize)]
struct TransactionRequest {
    sender: String,
    recipient: String,
    amount: u64,
}

#[post("/transactions/new", format = "json", data = "<body>")]
fn new_transaction(body: Json<TransactionRequest>, state: State<Mutex<Chain>>) -> Response {
    let mut chain = state.lock().unwrap();
    let index = chain.new_transaction(body.0.sender, body.0.recipient, body.0.amount);
    let transaction = chain.current_transactions().get(index as usize);
    let result = json!({
             "message" : "Transaction added to the block",
             "transaction" : transaction
        }).to_string();
    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(result))
        .finalize();
    response
}

#[get("/nodes/resolve")]
fn resolve(state: State<Mutex<Chain>>) -> Response {
    let mut chain = state.lock().unwrap();
    let result = chain.resolve_conflicts();
    let message = if result { "Our chain was replaced" } else { "Our chain is authoritative" };
    let result = json!({
             "message" : message.to_string(),
             "chain" : chain.to_json()
        }).to_string();
    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(result))
        .finalize();
    response
}

#[derive(Serialize, Deserialize)]
struct NodeRequest {
    nodes: Vec<String>
}

#[post("/nodes/register", format = "json", data = "<body>")]
fn register_node(body: Json<NodeRequest>, state: State<Mutex<Chain>>) -> Response {
    let mut chain = state.lock().unwrap();
    body.nodes.iter().for_each(|node| chain.register_node(node));

    let result = json!({
             "message" : format!("New nodes have been added"),
             "nodes": chain.nodes()
        }).to_string();
    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(result))
        .finalize();
    response
}

#[get("/nodes")]
fn nodes(state: State<Mutex<Chain>>) -> JsonValue {
    let chain = state.lock().unwrap();
    json!(chain.nodes())
}

/**
Starting the web server:

We will deploy the server at https://0.0.0.0:5000/
*/
pub fn rocket() -> rocket::Rocket {
    let chain = Mutex::new(Chain::new());
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(5000)
        .finalize().unwrap();

    rocket::custom(config)
        .manage(chain)
        // .register(catchers![not_found, internal_error])
        .mount("/", routes![index, chain, mine, new_transaction, resolve, register_node, nodes])
}
