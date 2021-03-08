use crate::blockchain::Chain;
use rocket::*;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use std::{sync::Mutex};
use rocket::config::{Config, Environment};
use rocket::response::{status, Redirect, Response};
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
        .sized_body(Cursor::new(chain.to_json()))
        .finalize();
    response
}

#[get("/mine")]
fn mine(state: State<Mutex<Chain>>) -> Response {
    let mut chain = state.lock().unwrap();
    let result = match chain.last_block() {
        Some(last_block) => {
            let last_proof = last_block.proof();
            let proof = Chain::proof_of_work(last_proof);
            let previous_hash = Chain::hash(last_block);
            chain.new_block(Some(previous_hash), proof);
            chain.to_json()
        }
        None => "502 Internal Server error".to_string(),
    };

    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(result))
        .finalize();
    response
}

#[derive(Serialize, Deserialize)]
struct TransactionRequest {
    sender: String,
    recipient: String,
    amount: u64,
}

#[post("/transactions/new", format = "json", data = "<body>")]
fn new_transaction(body: Option<Json<TransactionRequest>>, state: State<Mutex<Chain>>) -> Response {
    let mut chain = state.lock().unwrap();
    let mut response_builder = Response::build();
    match body {
        Some(json) => {
            let index = chain.new_transaction(json.0.sender, json.0.recipient, json.0.amount);
            let result = json!({
             "message" : format!("Transaction will be added to the block {}", index),
        }).to_string();
            response_builder
                .status(Status::Ok)
                .header(ContentType::JSON)
                .sized_body(Cursor::new(result))
                .finalize()
        }
        None => response_builder
            .status(Status::Ok)
            .header(ContentType::HTML)
            .sized_body(Cursor::new("400 Bad request")).finalize()
    }
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
    body.nodes.iter().for_each(|node| chain.register_node(node));
    status::Accepted(Some("nodes added".to_string()))
}

#[get("/nodes")]
fn nodes(state: State<Mutex<Chain>>) -> JsonValue {
    let chain = state.lock().unwrap();
    json!(chain.nodes())
}

// #[catch(500)]
// fn internal_error() -> &'static str {
//     "Whoops! Looks like we messed up."
// }
//
// #[catch(404)]
// fn not_found(req: &Request) -> String {
//     format!("I couldn't find '{}'. Try something else?", req.uri())
// }

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
