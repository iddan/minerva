use crate::quad::Quad;
use crate::term::{Identifier,IRI,Node,Literal,BlankNode};
use crate::namespace::XSD;

pub fn serialize_literal(literal: Literal) -> String {
    if literal.language.is_some() {
        return format!("\"{}\"@{}", literal.value, literal.language.unwrap())
    }
    if literal.datatype != XSD.iri("string") {
        return format!("\"{}\"^^{}", literal.value, serialize_iri(literal.datatype))
    }
    format!("\"{}\"", literal.value)
}

pub fn serialize_blank_node(blank_node: BlankNode) -> String {
    // TODO use explicit identifier
    format!("_:{}", blank_node.value)
}

pub fn serialize_iri(iri: IRI) -> String {
    format!("<{}>", iri.value)
}

pub fn serialize_identifier(identifier: Identifier) -> String {
    match identifier {
        Identifier::IRI(iri) => serialize_iri(iri),
        Identifier::BlankNode(blank_node) => serialize_blank_node(blank_node)
    }
}

pub fn serialize_node(node: Node) -> String {
    match node {
        Node::BlankNode(blank_node) => serialize_blank_node(blank_node),
        Node::IRI(iri) => serialize_iri(iri),
        Node::Literal(literal) => serialize_literal(literal),
    }
}

pub fn serialize_quad(quad: Quad) -> String {
    match quad.context {
        Some(identifier) => format!(
            "{} {} {} {} .",
            serialize_identifier(quad.subject),
            serialize_iri(quad.predicate),
            serialize_node(quad.object),
            serialize_identifier(identifier)
        ),
        None => format!(
            "{} {} {} .",
            serialize_identifier(quad.subject),
            serialize_iri(quad.predicate),
            serialize_node(quad.object),
        )
    }
}

pub fn serialize(iterator: impl Iterator<Item=Quad>) -> impl Iterator<Item=String> {
    iterator.map(|quad| format!("{}\n", serialize_quad(quad)))
}