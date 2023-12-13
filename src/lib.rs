// use tree_sitter::Parser;
// use wasm_bindgen::prelude::*;
// use tree_sitter_javascript::language;

// #[wasm_bindgen]
// pub fn concat(_left: Vec<String>, right: Vec<String>) -> Vec<String> {
//     let mut parser = Parser::new();
//     parser.set_language(language()).unwrap();
//     right.iter().map(|t| {
//         parser.parse(t, None).unwrap().root_node().to_sexp()
//     }).collect()
// }

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn concat(left: Vec<String>, right: Vec<String>) -> Vec<String> {
    left.iter().zip(right.iter()).map(|(l,r)| format!("{}*{}", l, r)).collect()
}