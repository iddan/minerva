use std::fs;
use std::collections::HashSet;
use crate::quad::Quad;
use crate::term::{BlankNode, Identifier, Literal, Node, IRI};

pub fn get_quads() -> HashSet<Quad> {
    let mut set: HashSet<Quad> = HashSet::new();
    set.extend(vec![
        Quad::new(
            Identifier::IRI(IRI {
                value: "http://example.com#tamir".to_owned(),
            }),
            IRI {
                value: "http://example.com#likes".to_owned(),
            },
            Node::IRI(IRI {
                value: "http://example.com#iddan".to_owned(),
            }),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
        Quad::new(
            Identifier::IRI(IRI {
                value: "http://example.com#tamir".to_owned(),
            }),
            IRI {
                value: "http://example.com#likes".to_owned(),
            },
            Node::BlankNode(BlankNode {
                value: "123".to_owned(),
            }),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
        Quad::new(
            Identifier::IRI(IRI {
                value: "http://example.com#iddan".to_owned(),
            }),
            IRI {
                value: "http://example.com#likes".to_owned(),
            },
            Node::IRI(IRI {
                value: "http://example.com#tamir".to_owned(),
            }),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
        Quad::new(
            Identifier::IRI(IRI {
                value: "http://example.com#tamir".to_owned(),
            }),
            IRI {
                value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_owned(),
            },
            Node::IRI(IRI {
                value: "http://example.com#Person".to_owned(),
            }),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
        Quad::new(
            Identifier::IRI(IRI {
                value: "http://example.com/test#lior".to_owned(),
            }),
            IRI {
                value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_owned(),
            },
            Node::IRI(IRI {
                value: "http://example.com#Person".to_owned(),
            }),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
        Quad::new(
            Identifier::BlankNode(BlankNode {
                value: "123".to_owned(),
            }),
            IRI {
                value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_owned(),
            },
            Node::IRI(IRI {
                value: "http://example.com#Person".to_owned(),
            }),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
        Quad::new(
            Identifier::IRI(IRI {
                value: "http://example.com#iddan".to_owned(),
            }),
            IRI {
                value: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_owned(),
            },
            Node::IRI(IRI {
                value: "http://example.com#Person".to_owned(),
            }),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
        Quad::new(
            Identifier::BlankNode(BlankNode {
                value: "123".to_owned(),
            }),
            IRI {
                value: "http://www.w3.org/2000/01/rdf-schema#label".to_owned(),
            },
            Node::Literal(Literal::new("Henry", None, None)),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
        Quad::new(
            Identifier::BlankNode(BlankNode {
                value: "123".to_owned(),
            }),
            IRI {
                value: "http://www.w3.org/2000/01/rdf-schema#label".to_owned(),
            },
            Node::Literal(Literal::new("Hendrik", None, Some("nl".to_owned()))),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
        Quad::new(
            Identifier::BlankNode(BlankNode {
                value: "123".to_owned(),
            }),
            IRI {
                value: "http://www.w3.org/2000/01/rdf-schema#label".to_owned(),
            },
            Node::Literal(Literal::new("Heinrich", None, Some("de".to_owned()))),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
        Quad::new(
            Identifier::BlankNode(BlankNode {
                value: "123".to_owned(),
            }),
            IRI {
                value: "http://www.w3.org/2000/01/rdf-schema#label".to_owned(),
            },
            Node::Literal(Literal::new("Hei nrich", None, None)),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
        Quad::new(
            Identifier::BlankNode(BlankNode {
                value: "123".to_owned(),
            }),
            IRI {
                value: "http://www.w3.org/2000/01/rdf-schema#label".to_owned(),
            },
            Node::Literal(Literal::new("Hei \"nrich", None, None)),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
        Quad::new(
            Identifier::BlankNode(BlankNode {
                value: "123".to_owned(),
            }),
            IRI {
                value: "http://example.com#age".to_owned(),
            },
            Node::Literal(Literal::new(
                "20",
                Some(IRI::new("http://www.w3.org/2001/XMLSchema#integer")),
                None,
            )),
            Identifier::IRI(IRI {
                value: "http://example.com#ontology".to_owned(),
            }),
        ),
    ]);
    set
}

pub fn get_nquads_string() -> String {
    String::from_utf8(fs::read("src/test_set.nq").unwrap()).unwrap()
}
