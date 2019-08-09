use crate::quad::{Quad, Subject, Predicate, Object, Context};
use crate::term::{Identifier,IRI,Node,Literal,BlankNode};
use std::str::Chars;


fn deserialize_identifier(string: &str) -> Result<Identifier, String> {
    match string.chars().next() {
        Some('<') => {
            let iri = deserialize_iri(&string).unwrap();
            Ok(Identifier::IRI(iri))
        },
        Some('_') => {
            let blank_node = deserialize_blank_node(&string).unwrap();
            Ok(Identifier::BlankNode(blank_node))
        },
        Some(c) => {
            // TODO: better error
            Err(format!("Unexpected character {}", c))
        },
        None => {
            Err("Unexpected EOF".to_owned())
        }
    }
}


fn deserialize_blank_node(string: &str) -> Result<BlankNode, String> {
    if !string.starts_with("_:") {
        // TODO: better error
        return Err(format!("Unexpected characters {}", &string[..2]));
    }
    let value = string.trim_start_matches("_:");
    if value.is_empty() {
        return Err("Unexpected EOF".to_owned());
    }
    Ok(BlankNode::from_value(value.to_owned()))
}

fn deserialize_iri(string: &str) -> Result<IRI, String> {
    if !string.starts_with("<") {
        // TODO: better error
        return Err(format!("Unexpected character {}", &string[..1]))
    }
    if !string.ends_with(">") {
        // TODO: better error
        return Err(format!("Unexpected character {}", &string[string.len() - 1..]))
    }
    Ok(IRI::new(string.trim_start_matches("<").trim_end_matches(">")))
}


fn deserialize_literal(string: &str) -> Result<Literal, String> {
    if !string.starts_with("\"") {
        return Err(format!("Unexpected character {}", &string[..1]));
    }
    let mut accumulator = String::new();
    let mut has_datatype = false;
    let mut has_language = false;
    let mut value: Option<String> = None;
    let mut datatype: Option<IRI> = None;
    let mut language: Option<String> = None;
    let mut chars = string.chars().skip(1);
    loop {
        match chars.next() {
            Some('"') => {
                if value.is_none() {
                    value = Some(accumulator.clone());
                    accumulator = String::new();
                }
                else {
                    return Err("Unexpected character \"".to_owned());
                }
            },
            Some('^') => {
                if value.is_some() && !has_datatype && !has_language && chars.next() == Some('^') {
                    has_datatype = true;
                }
                else {
                    return Err("Unexpected character ^".to_owned());
                }
            },
            Some('@') => {
                if value.is_some() && !has_datatype && !has_language {
                    has_language = true;
                }
                else {
                    return Err("Unexpected character @".to_owned());
                }
            },
            Some(c) => {
                if value.is_none() || !has_datatype || !has_language {
                    accumulator.push(c);
                }
                else {
                    return Err(format!("Unexpected character {}", c));
                }
            },
            None => {
                if has_datatype {
                    if accumulator.is_empty() {
                        return Err("Unexpected EOF".to_owned());
                    }
                    let iri = deserialize_iri(&accumulator).unwrap();
                    datatype = Some(iri);
                }
                else if has_language {
                    if accumulator.is_empty() {
                        return Err("Unexpected EOF".to_owned());
                    }
                    language = Some(accumulator.clone());
                }
                return Ok(Literal::new(value.unwrap(), datatype, language))
            }
        }
    }
}

fn deserialize_node(string: &str) -> Result<Node, String> {
    match string.chars().next() {
        Some('<') => {
            let iri = deserialize_iri(&string)?;
            Ok(Node::IRI(iri))
        },
        Some('_') => {
            let blank_node = deserialize_blank_node(&string)?;
            Ok(Node::BlankNode(blank_node))
        }
        Some('"') => {
            let literal = deserialize_literal(&string)?;
            Ok(Node::Literal(literal))
        }
        Some(c) => {
            // TODO: better error
            Err(format!("Unexpected character {}", c))
        }
        None => {
            Err("Unexpected EOF".to_owned())
        }
    }
}


pub struct NQuadsDeserializer<'a> {
    chars: Chars<'a>,
    column: u32,
    line: u32
}


impl <'a>NQuadsDeserializer<'a> {
    pub fn new(nquads: &'a str) -> NQuadsDeserializer<'a> {
        return NQuadsDeserializer { column: 0, line: 1, chars: nquads.chars() };
    }
}


impl <'a>Iterator for NQuadsDeserializer<'a> {
    type Item = Result<Quad, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut accumulator = String::new();
        let mut subject: Option<Subject> = None;
        let mut predicate: Option<Predicate> = None;
        let mut object: Option<Object> = None;
        let mut context: Option<Context> = None;
        let line = self.line;
        let column = self.column;

        // TODO correct line column
        let wrap_err = |error: String| -> String {
            format!("At line {} column {}: {}", line, column, error)
        };

        let result: Result<Option<Quad>, String> = try {
            loop {
                self.column += 1;
                match self.chars.next() {
                    Some('\n') => {
                        if accumulator.is_empty() {
                            self.line += 1;
                            self.column = 0;
                            continue
                        }
                        return Some(Err(wrap_err("Unexpected character \\n".to_owned())))
                    },
                    Some(' ') => {
                        if subject.is_none() {
                            let identifier = deserialize_identifier(&accumulator).map_err(&wrap_err)?;
                            subject = Some(identifier);
                        }
                        else if predicate.is_none() {
                            let iri = deserialize_iri(&accumulator).map_err(&wrap_err)?;
                            predicate = Some(iri);
                        }
                        else if object.is_none() {
                            let node = deserialize_node(&accumulator).map_err(&wrap_err)?;
                            object = Some(node);
                        }
                        else {
                            let identifier = deserialize_identifier(&accumulator).map_err(&wrap_err)?;
                            context = Some(identifier);
                        }
                        accumulator = String::new();
                    },
                    Some('.') => {
                        if context.is_some() || object.is_some() && accumulator.is_empty() {
                            return Some(Ok(Quad::new(subject.unwrap(), predicate.unwrap(), object.unwrap(), context.unwrap())))
                        }
                        accumulator.push('.');
                    },
                    Some(c) => {
                        accumulator.push(c);
                    }
                    None => {
                        return None
                    }
                }
            }
        };

        if result.is_ok() {
            let value = result.unwrap();
            if value.is_none() {
                None
            }
            else {
                Some(Ok(value.unwrap()))
            }
        }
        else {
            Some(Err(result.unwrap_err()))
        }
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::collections::HashSet;
    use crate::nquads_deserialize::NQuadsDeserializer;
    use crate::quad::Quad;
    use crate::term::{Identifier, Node, IRI, BlankNode, Literal};
    #[test]
    fn deserialize() {
        let nquads = String::from_utf8(fs::read("src/test.nq").unwrap()).unwrap();
        let deserializer = NQuadsDeserializer::new(&nquads);
        let quads_result: Result<HashSet<Quad>, _> = deserializer.collect();
        let quads = quads_result.unwrap();
        let mut set: HashSet<Quad> = HashSet::new();
        set.extend(vec![
                Quad {
                    subject: Identifier::IRI(IRI { value: "http://example.com#tamir".to_owned() }),
                    predicate: IRI { value: "http://example.com#likes".to_owned() },
                    object: Node::IRI(IRI { value: "http://example.com#iddan".to_owned() }),
                    context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_owned() }) 
                }, 
                Quad {
                    subject: Identifier::IRI(IRI { value: "http://example.com#tamir".to_owned() }),
                    predicate: IRI { value: "http://example.com#likes".to_owned() },
                    object: Node::BlankNode(BlankNode { value: "123".to_owned() }),
                    context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_owned() }) 
                }, 
                Quad {
                    subject: Identifier::IRI(IRI { value: "http://example.com#iddan".to_owned() }),
                    predicate: IRI { value: "http://example.com#likes".to_owned() },
                    object: Node::IRI(IRI { value: "http://example.com#tamir".to_owned() }),
                    context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_owned() }) 
                }, 
                Quad {
                    subject: Identifier::IRI(IRI { value: "http://example.com#tamir".to_owned() }),
                    predicate: IRI { value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_owned() },
                    object: Node::IRI(IRI { value: "http://example.com#Person".to_owned() }),
                    context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_owned() }) 
                }, 
                Quad {
                    subject: Identifier::IRI(IRI { value: "http://example.com/test#lior".to_owned() }),
                    predicate: IRI { value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_owned() },
                    object: Node::IRI(IRI { value: "http://example.com#Person".to_owned() }),
                    context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_owned() }) 
                }, 
                Quad {
                    subject: Identifier::BlankNode(BlankNode { value: "123".to_owned() }),
                    predicate: IRI { value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_owned() },
                    object: Node::IRI(IRI { value: "http://example.com#Person".to_owned() }),
                    context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_owned() }) 
                }, 
                Quad {
                    subject: Identifier::IRI(IRI { value: "http://example.com#iddan".to_owned() }),
                    predicate: IRI { value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_owned() },
                    object: Node::IRI(IRI { value: "http://example.com#Person".to_owned() }),
                    context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_owned() })
                },
                Quad {
                    subject: Identifier::BlankNode(BlankNode { value: "123".to_owned() }),
                    predicate: IRI { value: "http://www.w3.org/2000/01/rdf-schema#label".to_owned() },
                    object: Node::Literal(Literal::new("Henry", None, None)),
                    context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_owned() }),
                },
                Quad {
                    subject: Identifier::BlankNode(BlankNode { value: "123".to_owned() }),
                    predicate: IRI { value: "http://www.w3.org/2000/01/rdf-schema#label".to_owned() },
                    object: Node::Literal(Literal::new("Hendrik", None, Some("nl".to_owned()))),
                    context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_owned() }),
                },
                Quad {
                    subject: Identifier::BlankNode(BlankNode { value: "123".to_owned() }),
                    predicate: IRI { value: "http://www.w3.org/2000/01/rdf-schema#label".to_owned() },
                    object: Node::Literal(Literal::new("Heinrich", None, Some("de".to_owned()))),
                    context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_owned() }),
                },
                Quad {
                    subject: Identifier::BlankNode(BlankNode { value: "123".to_owned() }),
                    predicate: IRI { value: "http://example.com#age".to_owned() },
                    object: Node::Literal(Literal::new("20", Some(IRI::new("http://www.w3.org/2001/XMLSchema#integer")), None)),
                    context: Identifier::IRI(IRI { value: "http://example.com#ontology".to_owned() }),
                }
        ]);
        assert_eq!(quads, set);
    }
}