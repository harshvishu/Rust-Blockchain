// use serde::{Deserialize, Serialize};
// use crate::blockchain::Block;

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct NodeResponse {
//     chain: Vec<Block>,
//     length: usize,
// }
//
// impl NodeResponse {
//     pub fn to_json(&self) -> String {
//         serde_json::to_string_pretty(&self).unwrap_or("".to_string())
//     }
// }
