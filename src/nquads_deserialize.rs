use crate::quad::{Context, Object, Predicate, Quad, Subject};
use crate::term::{BlankNode, Identifier, Literal, Node, IRI};
use std::iter::Peekable;
use std::str::Chars;

fn deserialize_blank_node(
    chars: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<BlankNode, String> {
    for expected_char in "_:".chars() {
        match chars.next() {
            Some(c) if c != expected_char => {
                return Err(format!("Unexpected character {}", c));
            }
            None => {
                return Err("Unexpected EOF".to_owned());
            }
            _ => {}
        }
    }
    let mut accumulator = String::new();
    loop {
        match chars.next() {
            None if accumulator.is_empty() => {
                return Err("Unexpected EOF".to_owned());
            }
            Some(' ') | None => {
                return Ok(BlankNode::from_value(accumulator));
            }
            Some(c) => {
                accumulator.push(c);
            }
        }
    }
}

fn deserialize_iri(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<IRI, String> {
    match chars.next() {
        Some(c) if c != '<' => {
            return Err(format!("Unexpected character {}", c));
        }
        None => {
            return Err("Unexpected EOF".to_owned());
        }
        _ => {}
    }
    let mut accumulator = String::new();
    loop {
        match chars.next() {
            Some('>') => {
                return Ok(IRI::new(accumulator));
            }
            Some(c) => {
                accumulator.push(c);
            }
            None => {
                return Err("Unexpected EOF".to_owned());
            }
        }
    }
}

fn deserialize_datatype(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<IRI, String> {
    for expected_char in "^^".chars() {
        let c = chars.next();
        match c {
            Some(c) if c == expected_char => {}
            Some(c) => {
                return Err(format!("Unexpected character {}", c));
            }
            None => {
                return Err("Unexpected EOF".to_owned());
            }
        }
    }
    return deserialize_iri(chars);
}

fn deserialize_language(
    chars: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<String, String> {
    let c = chars.next();
    match c {
        Some('@') => {}
        Some(c) => {
            return Err(format!("Unexpected character {}", c));
        }
        None => {
            return Err("Unexpected EOF".to_owned());
        }
    };
    let mut accumulator = String::new();
    loop {
        match chars.peek() {
            Some(' ') | Some('.') | None => return Ok(accumulator),
            Some(_) => {
                accumulator.push(chars.next().unwrap());
            }
        }
    }
}

fn deserialize_literal_value(
    chars: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<String, String> {
    match chars.next() {
        Some('"') => {}
        Some(c) => {
            return Err(format!("Unexpected character {}", c));
        }
        None => {
            return Err("Unexpected EOF".to_owned());
        }
    };
    let mut accumulator = String::new();
    let mut escaped = false;
    loop {
        match chars.next() {
            Some('\\') => {
                escaped = !escaped;
            }
            Some('"') if !escaped => {
                return Ok(accumulator);
            }
            Some(c) => {
                accumulator.push(c);
                if escaped {
                    escaped = false;
                }
            }
            None => return Err("Unexpected EOF".to_owned()),
        }
    }
}

fn deserialize_literal(
    chars: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<Literal, String> {
    let value = deserialize_literal_value(chars)?;
    // TODO make functions do this:
    match chars.peek() {
        Some('^') => {
            let datatype = deserialize_datatype(chars)?;
            Ok(Literal::new(value, datatype, None))
        }
        Some('@') => {
            let language = deserialize_language(chars)?;
            Ok(Literal::new(value, None, language))
        }
        _ => Ok(Literal::new(value, None, None)),
    }
}

fn deserialize_identifier(
    chars: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<Identifier, String> {
    match chars.peek() {
        Some('<') => {
            let iri = deserialize_iri(chars)?;
            Ok(Identifier::IRI(iri))
        }
        Some('_') => {
            let blank_node = deserialize_blank_node(chars)?;
            Ok(Identifier::BlankNode(blank_node))
        }
        Some(c) => Err(format!("Unexpected character {}", c)),
        None => Err("Unexpected EOF".to_owned()),
    }
}

fn deserialize_node(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<Node, String> {
    match chars.peek() {
        Some('<') => {
            let iri = deserialize_iri(chars)?;
            Ok(Node::IRI(iri))
        }
        Some('"') => {
            let literal = deserialize_literal(chars)?;
            Ok(Node::Literal(literal))
        }
        Some('_') => {
            let blank_node = deserialize_blank_node(chars)?;
            Ok(Node::BlankNode(blank_node))
        }
        Some(c) => Err(format!("Unexpected character {}", c)),
        None => Err("Unexpected EOF".to_owned()),
    }
}

pub struct NQuadsDeserializer<I: Iterator<Item = char>> {
    chars: Peekable<I>,
    column: u32,
    line: u32,
}

impl<I: Iterator<Item = char>> Iterator for NQuadsDeserializer<I> {
    type Item = Result<Quad<'static>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut subject: Option<Subject> = None;
        let mut predicate: Option<Predicate> = None;
        let mut object: Option<Object> = None;
        let mut context: Option<Context> = None;

        let result: Result<Option<Quad>, String> = try {
            loop {
                self.column += 1;

                match self.chars.peek() {
                    Some('\n') => {
                        if subject.is_some() {
                            return Some(Err("Unexpected character \\n".to_owned()));
                        }
                        self.line += 1;
                        self.column = 0;
                        self.chars.next();
                        continue;
                    }
                    Some(' ') => {
                        self.column += 1;
                        self.chars.next();
                    }
                    Some('.') => {
                        if object.is_some() {
                            self.chars.next();
                            if context.is_some() {
                                return Some(Ok(Quad::new(
                                    subject.unwrap(),
                                    predicate.unwrap(),
                                    object.unwrap(),
                                    context.unwrap(),
                                )));
                            }
                            return Some(Ok(Quad::new(
                                subject.unwrap(),
                                predicate.unwrap(),
                                object.unwrap(),
                                None,
                            )));
                        }
                        return Some(Err("Unexpected character .".to_owned()));
                    }
                    Some(_) => {
                        if subject.is_none() {
                            let identifier = deserialize_identifier(&mut self.chars)?;
                            subject = Some(&identifier);
                        } else if predicate.is_none() {
                            let iri = deserialize_iri(&mut self.chars)?;
                            predicate = Some(&iri);
                        } else if object.is_none() {
                            let node = deserialize_node(&mut self.chars)?;
                            object = Some(&node);
                        } else {
                            let identifier = deserialize_identifier(&mut self.chars)?;
                            context = Some(Some(&identifier));
                        }
                    }
                    None => return None,
                }
            }
        };

        if result.is_ok() {
            let value = result.unwrap();
            if value.is_none() {
                None
            } else {
                Some(Ok(value.unwrap()))
            }
        } else {
            let error = result.unwrap_err();
            let wrapped_err = format!("At line {} column {}: {}", self.line, self.column, error);
            Some(Err(wrapped_err))
        }
    }
}

pub fn deserialize<'a>(nquads: &'a str) -> NQuadsDeserializer<Chars<'a>> {
    return NQuadsDeserializer {
        column: 0,
        line: 1,
        chars: nquads.chars().peekable(),
    };
}

#[cfg(test)]
mod tests {
    use crate::nquads_deserialize::deserialize;
    use crate::quad::Quad;
    use crate::test_set;
    use std::collections::HashSet;
    use std::fs;
    #[test]
    fn test_deserialize() {
        let nquads = String::from_utf8(fs::read("src/test_set.nq").unwrap()).unwrap();
        let quads_result: Result<HashSet<Quad>, _> = deserialize(&nquads).collect();
        let quads = quads_result.unwrap();
        let set = test_set::get();
        assert_eq!(quads, set);
    }
}
