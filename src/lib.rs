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
            ws.clone().into(),
            multiplication.into(),
        ],
    ));

    eprintln!("addition = {:?}", addition);
    eprintln!("{}", addition);
}
