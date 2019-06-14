#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Literal {
    value: &'static str,
    datatype: Option<&'static str>,
    language: Option<&'static str>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct IRI {
    value: &'static str,
}

impl IRI {
    pub fn from(value: &'static str) -> IRI {
        IRI { value }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct BlankNode;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Node {
    IRI(IRI),
    BlankNode(BlankNode),
    Literal(Literal),
}

impl From<IRI> for Node {
    fn from(node: IRI) -> Self {
        Node::IRI(node)
    }
}

impl From<BlankNode> for Node {
    fn from(node: BlankNode) -> Self {
        Node::BlankNode(node)
    }
}

impl From<Literal> for Node {
    fn from(node: Literal) -> Self {
        Node::Literal(node)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Identifier {
    IRI(IRI),
    BlankNode(BlankNode),
}

impl From<IRI> for Identifier {
    fn from(node: IRI) -> Self {
        Identifier::IRI(node)
    }
}

impl From<BlankNode> for Identifier {
    fn from(node: BlankNode) -> Self {
        Identifier::BlankNode(node)
    }
}