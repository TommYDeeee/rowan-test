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

#![forbid(unconditional_recursion, future_incompatible)]
#![deny(unsafe_code)]
#![feature(offset_of)]

#[allow(unsafe_code)]
mod green;
#[allow(unsafe_code)]
pub mod red;

pub mod api;
mod syntax_text;
mod utility_types;

#[allow(unsafe_code)]
mod arc;
pub mod ast;
mod cow_mut;
#[allow(unsafe_code)]
mod sll;

pub use text_size::{TextLen, TextRange, TextSize};

pub use crate::{
    api::{
        Language, SyntaxElement, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
    },
    green::{
        Checkpoint, Children, GreenNode, GreenNodeBuilder, GreenNodeData, GreenToken,
        GreenTokenData, NodeCache, SyntaxKind,
    },
    syntax_text::SyntaxText,
    utility_types::{Direction, NodeOrToken, TokenAtOffset, WalkEvent},
};
