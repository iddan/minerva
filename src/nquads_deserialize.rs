use crate::quad::{Quad, Subject, Predicate, Object, Context};
use crate::term::{Identifier,IRI,Node,Literal,BlankNode};
use std::str::Chars;

fn deserialize_identifier(string: &str) -> Identifier {
    match string.chars().next() {
        Some('<') => {
            return Identifier::IRI(deserialize_iri(&string))
        },
        Some('_') => {
            return Identifier::BlankNode(deserialize_blank_node(&string))
        }
        _ => {
            
        }
    }
    panic!("oh no {}", string)
}

fn deserialize_blank_node(string: &str) -> BlankNode {
    BlankNode::from_value(string.trim_start_matches("_:").to_owned())
}

fn deserialize_iri(string: &str) -> IRI {
    IRI::new(string.trim_start_matches("<").trim_end_matches(">"))
}

fn deserialize_literal(string: &str) -> Literal {
    Literal::new(string.trim_start_matches("\""), None, None)
}

fn deserialize_node(string: &str) -> Node {
    match string.chars().next() {
        Some('<') => {
            return Node::IRI(deserialize_iri(&string))
        },
        Some('_') => {
            return Node::BlankNode(deserialize_blank_node(&string))
        }
        Some('"') => {
            return Node::Literal(deserialize_literal(&string))
        }
        _ => {

        }
    }
    panic!("oh no {}", string)
}


pub struct NQuadsDeserializer {
    chars: Chars<'static>,
}

impl Iterator for NQuadsDeserializer {
    type Item = Quad;
    fn next(&mut self) -> Option<Quad> {
        let mut accumulator = String::new();
        let mut subject: Option<Subject> = None;
        let mut predicate: Option<Predicate> = None;
        let mut object: Option<Object> = None;
        loop {
            match self.chars.next() {
                Some('\n') | None => {
                    if accumulator.is_empty() {
                        return None
                    }
                    let context: Option<Context> = Some(deserialize_identifier(&accumulator));
                    return Some(Quad::new(subject.unwrap(), predicate.unwrap(), object.unwrap(), context.unwrap()))
                },
                Some(' ') => {
                    if subject.is_none() {
                        subject = Some(deserialize_identifier(&accumulator));
                    }
                    else if predicate.is_none() {
                        predicate = Some(deserialize_iri(&accumulator));
                    }
                    else if object.is_none() {
                        object = Some(deserialize_node(&accumulator));
                    }
                    accumulator = String::new();
                },
                Some(c) => {
                    accumulator.push(c);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::nquads_deserialize::NQuadsDeserializer;
    use crate::quad::Quad;
    use crate::term::{Identifier, Node, IRI};
    #[test]
    fn deserialize() {
        let nquads = "<http://example.com/test#lior> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://example.com#Person> <http://example.com#ontology>
<http://example.com#iddan> <http://example.com#likes> <http://example.com#tamir> <http://example.com#ontology>
<http://example.com#tamir> <http://example.com#likes> <http://example.com#iddan> <http://example.com#ontology>
<http://example.com#iddan> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://example.com#Person> <http://example.com#ontology>
<http://example.com#tamir> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://example.com#Person> <http://example.com#ontology>
<http://example.com#tamir> <http://example.com#likes> <_:123> <http://example.com#ontology>
<_:123> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://example.com#Person> <http://example.com#ontology>";
        let deserializer = NQuadsDeserializer { chars: nquads.chars() };
        let quads: HashSet<Quad> = deserializer.collect();
        assert_eq!(quads.len(), 5);
        let set: HashSet<Quad> = HashSet::new();
        set.extend(&[
            Quad {
                subject: Identifier::IRI(IRI { value: "http://example.com#iddan".to_string() }),
                predicate: IRI { value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_string() },
                object: Node::IRI(IRI { value: "http://example.com#Person".to_string() }),
                context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_string() })
            },
            Quad {
                subject: Identifier::IRI(IRI { value: "http://example.com#tamir".to_string() }),
                predicate: IRI { value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_string() },
                object: Node::IRI(IRI { value: "http://example.com#Person".to_string() }),
                context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_string() })
            },
            Quad {
                subject: Identifier::IRI(IRI { value: "http://example.com#tamir".to_string() }),
                predicate: IRI { value: "http://example.com#likes".to_string() },
                object: Node::IRI(IRI { value: "http://example.com#iddan".to_string() }),
                context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_string() })
            },
            Quad {
                subject: Identifier::IRI(IRI { value: "http://example.com#iddan".to_string() }),
                predicate: IRI { value: "http://example.com#likes".to_string() },
                object: Node::IRI(IRI { value: "http://example.com#tamir".to_string() }),
                context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_string() })
            },
            Quad {
                subject: Identifier::IRI(IRI { value: "http://example.com/test#lior".to_string() }),
                predicate: IRI { value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_string() },
                object: Node::IRI(IRI { value: "http://example.com#Person".to_string() }),
                context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_string() })
            }
        ]);
        assert_eq!(quads, set);
    }
}