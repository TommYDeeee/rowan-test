use std::rc::Rc;

use crate::{GreenNode, GreenToken, NodeOrToken, SyntaxKind};

pub type RedNode = Rc<RedNodeData>;
#[derive(Clone)]
pub struct RedNodeData {
    parent: Option<RedNode>,
    green: GreenNode,
    text_offset: usize,
}

pub type RedToken = Rc<RedTokenData>;
#[derive(Clone)]
pub struct RedTokenData {
    parent: Option<RedNode>,
    green: GreenToken,
    text_offset: usize,
}

type RedElement = NodeOrToken<RedNode, RedToken>;

impl RedNodeData {
    pub fn new(root: GreenNode) -> RedNode {
        Rc::new(RedNodeData {
            parent: None,
            green: root,
            text_offset: 0,
        })
    }

    pub fn green(&self) -> &GreenNode {
        &self.green
    }

    pub fn kind(&self) -> SyntaxKind {
        self.green().kind()
    }

    pub fn text_len(&self) -> usize {
        self.green().text_len()
    }

    pub fn text_offset(&self) -> usize {
        self.text_offset
    }

    pub fn parent(&self) -> Option<&RedNode> {
        self.parent.as_ref()
    }
}

impl RedTokenData {
    pub fn green(&self) -> &GreenToken {
        &self.green
    }

    pub fn kind(&self) -> SyntaxKind {
        self.green().kind()
    }

    pub fn text_len(&self) -> usize {
        self.green().text_len()
    }

    pub fn text_offset(&self) -> usize {
        self.text_offset
    }

    pub fn parent(&self) -> Option<&RedNode> {
        self.parent.as_ref()
    }
}
