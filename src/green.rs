mod builder;
mod element;
mod node;
mod token;

use self::element::GreenElement;

pub(crate) use self::{element::GreenElementRef, node::GreenChild};

pub use self::{
    builder::{Checkpoint, GreenNodeBuilder, NodeCache},
    node::{Children, GreenNode, GreenNodeData},
    token::{GreenToken, GreenTokenData},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyntaxKind(pub u16);
