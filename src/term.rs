use crate::sum_type;

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

impl From<&'static str> for IRI {
    fn from(value: &'static str) -> IRI {
        IRI { value }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct BlankNode;

sum_type!(#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)] pub enum Node, IRI, BlankNode, Literal);

sum_type!(#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)] pub enum Identifier, IRI, BlankNode);
