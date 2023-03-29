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

use wasm_bindgen::{prelude::*};
use serde::{Deserialize, Serialize};
use rayon::prelude::*;
#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
pub fn generate() -> String {
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Edge {
        pub et: String, // edge_type
        pub tn: usize,  // to_node
        pub ni: String, // name_or_index
        #[serde(skip_serializing)]
        pub isw: usize, // is_weak_retainer 0 false 1 true
        pub isr: usize, // is_retainer 0 false 1 true
    }
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
        pub edges: Vec<Edge>,
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
    console_log!("{}", str);
    return str;
}
