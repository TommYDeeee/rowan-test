mod builder;
mod element;
mod node;
mod node_cache;
mod token;

use self::element::GreenElement;

pub(crate) use self::{element::GreenElementRef, node::GreenChild};

pub use self::{
    builder::{Checkpoint, GreenNodeBuilder},
    node::{Children, GreenNode, GreenNodeData},
    node_cache::NodeCache,
    token::{GreenToken, GreenTokenData},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyntaxKind(pub u16);
