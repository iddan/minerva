use serde::{Deserialize, Serialize};
use crate::namespace::XSD;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct IRI {
    pub value: String,
}

impl IRI {
    pub fn new<V>(value: V) -> IRI
    where
        V: Into<String>,
    {
        IRI {
            value: value.into(),
        }
    }
}

impl From<&IRI> for IRI {
    fn from(iri: &IRI) -> IRI {
        iri.to_owned()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BlankNode;

impl BlankNode {
    pub fn new() -> BlankNode {
        BlankNode {}
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Literal {
    pub value: String,
    pub datatype: IRI,
    pub language: Option<String>,
}

impl Literal {
    pub fn new<V, D, L>(value: V, datatype: D, language: L) -> Literal
    where
        V: Into<String>,
        D: Into<Option<IRI>>,
        L: Into<Option<String>>,
    {
        Literal {
            value: value.into(),
            datatype: datatype.into().unwrap_or(XSD.iri("string")),
            language: language.into(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Node {
    IRI(IRI),
    BlankNode(BlankNode),
    Literal(Literal),
}

impl From<IRI> for Node {
    fn from(value: IRI) -> Node {
        Node::IRI(value)
    }
}

impl From<BlankNode> for Node {
    fn from(value: BlankNode) -> Node {
        Node::BlankNode(value)
    }
}

impl From<Literal> for Node {
    fn from(value: Literal) -> Node {
        Node::Literal(value)
    }
}

impl From<&IRI> for Node {
    fn from(value: &IRI) -> Node {
        Node::IRI(value.to_owned())
    }
}

impl From<&BlankNode> for Node {
    fn from(value: &BlankNode) -> Node {
        Node::BlankNode(value.to_owned())
    }
}

impl From<&Literal> for Node {
    fn from(value: &Literal) -> Node {
        Node::Literal(value.to_owned())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Identifier {
    IRI(IRI),
    BlankNode(BlankNode),
}

impl From<IRI> for Identifier {
    fn from(value: IRI) -> Identifier {
        Identifier::IRI(value)
    }
}

impl From<BlankNode> for Identifier {
    fn from(value: BlankNode) -> Identifier {
        Identifier::BlankNode(value)
    }
}

impl From<&IRI> for Identifier {
    fn from(value: &IRI) -> Identifier {
        Identifier::IRI(value.to_owned())
    }
}

impl From<&BlankNode> for Identifier {
    fn from(value: &BlankNode) -> Identifier {
        Identifier::BlankNode(value.to_owned())
    }
}
