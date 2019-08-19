use crate::namespace::XSD;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BlankNode {
    pub value: String,
}

// TODO: generate real unique
fn generate_blank_node_id() -> String {
    Uuid::new_v4().to_string()
}

impl BlankNode {
    pub fn new() -> BlankNode {
        BlankNode {
            value: generate_blank_node_id(),
        }
    }

    pub fn from_value<V>(value: V) -> BlankNode
    where
        V: Into<String>,
    {
        BlankNode {
            value: value.into(),
        }
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

impl<'a> From<&'a IRI> for &'a Identifier {
    fn from(value: &'a IRI) -> &'a Identifier {
        &Identifier::IRI(value.to_owned())
    }
}

impl<'a> From<&'a BlankNode> for &'a Identifier {
    fn from(value: &'a BlankNode) -> &'a Identifier {
        &Identifier::BlankNode(value.to_owned())
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

impl<'a> From<&'a IRI> for &'a Node {
    fn from(value: &'a IRI) -> &'a Node {
        &Node::IRI(value.to_owned())
    }
}

impl<'a> From<&'a BlankNode> for &'a Node {
    fn from(value: &'a BlankNode) -> &'a Node {
        &Node::BlankNode(value.to_owned())
    }
}

impl<'a> From<&'a Literal> for &'a Node {
    fn from(value: &'a Literal) -> &'a Node {
        &Node::Literal(value.to_owned())
    }
}

impl<'a> From<&'a Identifier> for &'a Node {
    fn from(value: &'a Identifier) -> &'a Node {
        match value {
            Identifier::IRI(iri) => iri.into(),
            Identifier::BlankNode(blank_node) => blank_node.into(),
        }
    }
}

pub fn node_to_identifier<'a>(node: &'a Node) -> Result<&'a Identifier, &'static str> {
    match node {
        Node::IRI(iri) => Ok(&Identifier::IRI(iri.to_owned())),
        Node::BlankNode(blank_node) => Ok(&Identifier::BlankNode(blank_node.to_owned())),
        Node::Literal(literal) => Err("Node::Literal can not be converted to Identifier"),
    }
}
