use crate::{NodeOrToken, SyntaxKind};
use std::{fmt, iter, sync::Arc};

pub type GreenToken = Arc<GreenTokenData>;
#[derive(Debug)]
pub struct GreenTokenData {
    kind: SyntaxKind,
    text: String,
}

pub type GreenNode = Arc<GreenNodeData>;
#[derive(Debug)]
pub struct GreenNodeData {
    kind: SyntaxKind,
    children: Vec<GreenElement>,
    len: usize,
}

pub type GreenElement = NodeOrToken<GreenNode, GreenToken>;

impl GreenTokenData {
    pub fn new(kind: SyntaxKind, text: String) -> GreenTokenData {
        GreenTokenData { kind, text }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn text_len(&self) -> usize {
        self.text().len()
    }
}

impl fmt::Display for GreenTokenData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.text(), f)
    }
}

impl GreenNodeData {
    pub fn new(kind: SyntaxKind, children: Vec<GreenElement>) -> GreenNodeData {
        let len = children.iter().map(|it| it.text_len()).sum();

        GreenNodeData {
            kind,
            children,
            len,
        }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn text_len(&self) -> usize {
        self.len
    }

    pub fn children(&self) -> &[GreenElement] {
        self.children.as_slice()
    }

    pub fn replace_child(&self, idx: usize, new_child: GreenElement) -> GreenNodeData {
        assert!(idx < self.children().len());

        let left_children = self.children().iter().take(idx).cloned();

        let right_children = self.children().iter().skip(idx + 1).cloned();

        let new_children = left_children
            .chain(iter::once(new_child))
            .chain(right_children)
            .collect();

        GreenNodeData::new(self.kind, new_children)
    }
}

impl From<GreenNode> for GreenElement {
    fn from(node: GreenNode) -> Self {
        NodeOrToken::Node(node)
    }
}

impl From<GreenToken> for GreenElement {
    fn from(token: GreenToken) -> Self {
        NodeOrToken::Token(token)
    }
}

impl fmt::Display for GreenNodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for child in self.children() {
            fmt::Display::fmt(child, f)?;
        }
        Ok(())
    }
}

impl GreenElement {
    pub fn text_len(&self) -> usize {
        match self {
            NodeOrToken::Node(it) => it.text_len(),
            NodeOrToken::Token(it) => it.text_len(),
        }
    }
}

impl fmt::Display for GreenElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeOrToken::Node(it) => fmt::Display::fmt(it, f),
            NodeOrToken::Token(it) => fmt::Display::fmt(it, f),
        }
    }
}
