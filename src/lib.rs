/*
 * Copyright 2022 Google Inc. All Rights Reserved.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *     http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use hsl::HSL;
use num_complex::Complex64;
use rand::Rng;
use wasm_bindgen::{prelude::*, Clamped};
use serde::{Deserialize, Serialize, Deserializer};
use rayon::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;


#[wasm_bindgen]
pub fn generate(width: u32, height: u32, max_iterations: u32) -> String {
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Node {
        pub nt: String,   // node_type
        pub name: String, // name
        pub id: usize,    // id
        pub size: usize,  // self_size
        #[serde(skip_serializing)]
        pub ec: usize, // edge_count
        // pub trace_node_id: JsValueType,
        pub rs: usize, // retained_size
        pub edges: Vec<usize>,
        pub p: Vec<usize>,
        pub c: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub se: Option<String>, // source
    }

    let v:Vec<Node> = (0..1000000).into_par_iter().map(|_| {
        let node = Node {
            nt: "1".to_string(),
            id: 1,
            ec: 1,
            edges: vec![],
            se: None,
            size: 1,
            name: "1".to_string(),
            p: vec![],
            rs: 1,
            c: "1".to_string(),
        };
       node
    }).collect();
    let str = serde_json::to_string(&v).unwrap();
    return str;
}
