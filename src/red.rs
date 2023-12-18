use core::fmt;
use std::{iter, rc::Rc, sync::Arc};

use crate::{green::GreenElement, GreenNode, GreenToken, NodeOrToken, SyntaxKind};

pub type RedNode = Rc<RedNodeData>;
#[derive(Clone)]
pub struct RedNodeData {
    parent: Option<RedNode>,
    index_in_parent: usize,
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

pub type RedElement = NodeOrToken<RedNode, RedToken>;

impl RedNodeData {
    pub fn new_root(root: GreenNode) -> RedNode {
        Rc::new(RedNodeData {
            parent: None,
            index_in_parent: 0,
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

    pub fn ancestors<'a>(self: &'a RedNode) -> impl Iterator<Item = &'a RedNode> {
        iter::successors(Some(self), |&it| it.parent())
    }

    // O(1)
    pub fn nth_child<'a>(self: &'a RedNode) -> Option<RedElement> {
        todo!()
    }

    // O(log(n))
    pub fn child_containing_range<'a>(self: &'a RedNode, range: (usize, usize)) -> Option<RedElement> {
        todo!()
    }

    pub fn children<'a>(self: &'a RedNode) -> impl Iterator<Item = RedElement> + 'a {
        let mut offset_in_parrent = 0;
        self.green()
            .children()
            .enumerate()
            .map(move |(index_in_parent, green_child)| {
                let text_offset = self.text_offset() + offset_in_parrent;
                offset_in_parrent += green_child.text_len();

                match green_child {
                    NodeOrToken::Node(node) => Rc::new(RedNodeData {
                        parent: Some(Rc::clone(self)),
                        index_in_parent,
                        green: node,
                        text_offset,
                    })
                    .into(),
                    NodeOrToken::Token(token) => Rc::new(RedTokenData {
                        parent: Some(Rc::clone(self)),
                        green: token,
                        text_offset,
                    })
                    .into(),
                }
            })
    }

    pub fn replace_child(self: &RedNode, idx: usize, new_child: GreenElement) -> RedNode {
        let new_green = self.green().replace_child(idx, new_child);

        self.replace_ourselves(Arc::new(new_green))
    }

    fn replace_ourselves(self: &RedNode, new_green: GreenNode) -> RedNode {
        match self.parent() {
            Some(parent) => parent.replace_child(self.index_in_parent, new_green.into()),
            None => RedNodeData::new_root(new_green),
        }
    }
}

impl fmt::Display for RedNodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.green(), f)
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

impl fmt::Display for RedTokenData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.green(), f)
    }
}

impl RedElement {
    pub fn kind(&self) -> SyntaxKind {
        match self {
            NodeOrToken::Node(it) => it.kind(),
            NodeOrToken::Token(it) => it.kind(),
        }
    }

    pub fn text_len(&self) -> usize {
        match self {
            NodeOrToken::Node(it) => it.text_len(),
            NodeOrToken::Token(it) => it.text_len(),
        }
    }

    pub fn text_offset(&self) -> usize {
        match self {
            NodeOrToken::Node(it) => it.text_offset(),
            NodeOrToken::Token(it) => it.text_offset(),
        }
    }

    pub fn parent(&self) -> Option<&RedNode> {
        match self {
            NodeOrToken::Node(it) => it.parent(),
            NodeOrToken::Token(it) => it.parent(),
        }
    }
}

impl fmt::Display for RedElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeOrToken::Node(it) => fmt::Display::fmt(it, f),
            NodeOrToken::Token(it) => fmt::Display::fmt(it, f),
        }
    }
}

impl From<RedNode> for RedElement {
    fn from(node: RedNode) -> Self {
        Self::Node(node)
    }
}

impl From<RedToken> for RedElement {
    fn from(token: RedToken) -> Self {
        Self::Token(token)
    }
}
