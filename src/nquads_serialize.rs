use std::error::Error;
use futures::stream::Stream;
use crate::quad::Quad;
use crate::term::{Identifier,IRI,Node,Literal,BlankNode};
use crate::namespace::XSD;

pub fn serialize_literal(literal: Literal) -> String {
    let escaped_value = literal.value.replace("\"", "\\\"");
    if literal.language.is_some() {
        return format!("\"{}\"@{}", escaped_value, literal.language.unwrap())
    }
    if literal.datatype != XSD.iri("string") {
        return format!("\"{}\"^^{}", escaped_value, serialize_iri(literal.datatype))
    }
    format!("\"{}\"", escaped_value)
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

pub fn serialize(stream: impl Stream<Item=Quad, Error=impl Error>) -> impl Stream<Item=String, Error=impl Error>  {
    stream.map(|quad| serialize_quad(quad))
}

mod tests {
    use std::fs;
    use std::collections::HashSet;
    use crate::nquads_serialize::serialize;
    use crate::test_set;
    use crate::quad::Quad;
    use futures::future::Future;
    use futures::stream::Stream;
    use std::error::Error;
    #[test]
    pub fn test_serialize() {
        let set = test_set::get();
        let nquads = String::from_utf8(fs::read("src/test_set.nq").unwrap()).unwrap();
        let mut nquads_set = HashSet::new();
        nquads_set.extend(nquads.split('\n').map(|s| {
            let mut s = s.to_owned();
            s.push('\n');
            s
        }));
        let test_set_stream = futures::stream::iter_ok::<_, dyn Error + Sized>(set.iter().map(|quad| quad.to_owned()));
        let mut result = serialize(test_set_stream).collect();
        let serialized_vec = result.wait().unwrap();
        let mut serialized = HashSet::new();
        serialized.extend(serialized_vec.iter().map(|s| s.to_owned()));
        println!("{:?}", serialized.difference(&nquads_set));
    }
}