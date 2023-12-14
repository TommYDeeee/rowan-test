/*
Custom implementation of rowan library - https://github.com/rust-analyzer/rowan
This structure should implement the following:
 * full fidelity - whitespace and comments are part of the tree
 * resilient & semi-structured - can parse incomplete code
 * cheaply updatable - refactors and incremental parsing
 * conviniently updatable
 *
 * immutable value type
 * easy to navigate - from node to children, parent, siblings
*/

mod green;
pub mod kinds;
mod red;

use std::sync::Arc;

pub use crate::{
    green::{GreenNode, GreenNodeData, GreenToken, GreenTokenData},
    red::{RedNode, RedNodeData, RedToken, RedTokenData},
};

#[derive(Clone, Copy, Debug)]
pub struct SyntaxKind(u16);

#[derive(Clone, Copy, Debug)]
pub enum NodeOrToken<N, T> {
    Node(N),
    Token(T),
}

impl<N, T> NodeOrToken<N, T> {
    pub fn into_node(self) -> Option<N> {
        match self {
            NodeOrToken::Node(it) => Some(it),
            NodeOrToken::Token(_) => None,
        }
    }

    pub fn into_token(self) -> Option<T> {
        match self {
            NodeOrToken::Node(_) => None,
            NodeOrToken::Token(it) => Some(it),
        }
    }
}

#[test]
fn smoke() {
    let ws = Arc::new(GreenTokenData::new(kinds::WHITESPACE, " ".to_string()));
    let one = Arc::new(GreenTokenData::new(kinds::INT, "1".to_string()));
    let plus = Arc::new(GreenTokenData::new(kinds::PLUS, "+".to_string()));
    let two = Arc::new(GreenTokenData::new(kinds::INT, "2".to_string()));
    let star = Arc::new(GreenTokenData::new(kinds::STAR, "*".to_string()));

    // 1 * 2
    let multiplication = Arc::new(GreenNodeData::new(
        kinds::BIN_EXPR,
        vec![
            one.into(),
            ws.clone().into(),
            star.into(),
            ws.clone().into(),
            two.into(),
        ],
    ));

    // 1 * 2 + 1 * 2
    let addition = Arc::new(GreenNodeData::new(
        kinds::BIN_EXPR,
        vec![
            multiplication.clone().into(),
            ws.clone().into(),
            plus.into(),
            ws.into(),
            multiplication.into(),
        ],
    ));

    eprintln!("addition = {:?}", addition);
    eprintln!("{}", addition);

    let addition = RedNodeData::new_root(addition);
    let mul2 = addition.children().nth(4).unwrap().into_node().unwrap();

    let three = Arc::new(GreenTokenData::new(kinds::INT, "3".to_string()));

    let new_root = mul2.replace_child(0, three.into());

    println!("{}", new_root);
}
