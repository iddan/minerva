// use rdf::graph::Graph;
// use rdf::uri::Uri;
// use rdf::triple::Triple;
// use petgraph::graphmap::DiGraphMap;
#[allow(dead_code)]
mod rdf {
    use std::collections::HashSet;
    use std::ops::{Add, AddAssign, Sub, SubAssign};

    trait Node {}

    trait Identifier {}

    #[derive(Debug)]
    pub struct Literal {
        value: &'static str,
        datatype: Option<&'static str>,
        language: Option<&'static str>,
    }

    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    pub struct IRI { value: &'static str }

    impl IRI {
        pub fn from(value: &'static str) -> IRI {
            IRI { value }
        }
    }

    #[derive(Debug)]
    pub struct BlankNode;

    impl Node for IRI {}
    impl Node for BlankNode {}
    impl Node for Literal {}

    impl Identifier for IRI {}
    impl Identifier for BlankNode {}

    // For simplicity sake let's keep it IRI triple until I'll understand how to type Nodes correctly.
    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    pub struct Quad {
        pub subject: IRI,
        pub predicate: IRI,
        pub object: IRI,
        pub context: IRI,
    }
    // struct Quad(Identifier, IRI, Node, Identifier);

    #[derive(Debug)]
    pub struct Dataset {
        quads: HashSet<Quad>,
    }

    impl Dataset {
        pub fn new() -> Dataset {
            Dataset {
                quads: HashSet::new(),
            }
        }
        pub fn insert(&mut self, quad: Quad) {
            self.quads.insert(quad);
        }
        pub fn contains(self, quad: &Quad) -> bool {
            return self.quads.contains(quad);
        }
        pub fn subjects(
            self,
            predicate: Option<IRI>,
            object: Option<IRI>,
            context: Option<IRI>,
        ) -> impl Iterator<Item = IRI> {
            self.quads
                .into_iter()
                .filter(move |quad| {
                    (predicate.map_or(true, |v| quad.predicate == v)
                        && object.map_or(true, |v| quad.object == v))
                        && context.map_or(true, |v| quad.context == v)
                })
                .map(|quad| quad.subject)
        }
        pub fn predicates(
            self,
            subject: Option<IRI>,
            object: Option<IRI>,
            context: Option<IRI>,
        ) -> impl Iterator<Item = IRI> {
            self.quads
                .into_iter()
                .filter(move |quad| {
                    subject.map_or(true, |v| quad.subject == v)
                        && object.map_or(true, |v| quad.object == v)
                        && context.map_or(true, |v| quad.context == v)
                })
                .map(|quad| quad.predicate)
        }
        pub fn objects(
            self,
            subject: Option<IRI>,
            predicate: Option<IRI>,
            context: Option<IRI>,
        ) -> impl Iterator<Item = IRI> {
            self.quads
                .into_iter()
                .filter(move |quad| {
                    subject.map_or(true, |v| quad.subject == v)
                        && predicate.map_or(true, |v| quad.predicate == v)
                        && context.map_or(true, |v| quad.context == v)
                })
                .map(|quad| quad.object)
        }
        pub fn subject_objects(
            self,
            predicate: Option<IRI>,
            context: Option<IRI>,
        ) -> impl Iterator<Item = (IRI, IRI)> {
            self.quads
                .into_iter()
                .filter(move |quad| {
                    predicate.map_or(true, |v| quad.predicate == v)
                        && context.map_or(true, |v| quad.context == v)
                })
                .map(|quad| (quad.subject, quad.object))
        }
        pub fn subject_predicates(
            self,
            object: Option<IRI>,
            context: Option<IRI>,
        ) -> impl Iterator<Item = (IRI, IRI)> {
            self.quads
                .into_iter()
                .filter(move |quad| {
                    object.map_or(true, |v| quad.object == v)
                        && context.map_or(true, |v| quad.context == v)
                })
                .map(|quad| (quad.subject, quad.predicate))
        }
        pub fn predicate_objects(
            self,
            subject: Option<IRI>,
            context: Option<IRI>,
        ) -> impl Iterator<Item = (IRI, IRI)> {
            self.quads
                .into_iter()
                .filter(move |quad| {
                    subject.map_or(true, |v| quad.subject == v)
                        && context.map_or(true, |v| quad.context == v)
                })
                .map(|quad| (quad.predicate, quad.object))
        }
    }

    impl IntoIterator for Dataset {
        type Item = Quad;
        type IntoIter = std::collections::hash_set::IntoIter<Quad>;

        fn into_iter(self) -> Self::IntoIter {
            self.quads.into_iter()
        }
    }

    impl Add for Dataset {
        type Output = Dataset;

        fn add(self, other: Dataset) -> Dataset {
            Dataset {
                quads: self.quads.union(&other.quads).cloned().collect(),
            }
        }
    }

    impl AddAssign for Dataset {
        fn add_assign(&mut self, other: Dataset) {
            *self = Dataset {
                quads: self.quads.union(&other.quads).cloned().collect(),
            };
        }
    }

    impl Sub for Dataset {
        type Output = Dataset;

        fn sub(self, other: Dataset) -> Dataset {
            Dataset {
                quads: self.quads.difference(&other.quads).cloned().collect(),
            }
        }
    }

    impl SubAssign for Dataset {
        fn sub_assign(&mut self, other: Dataset) {
            *self = Dataset {
                quads: self.quads.difference(&other.quads).cloned().collect(),
            };
        }
    }
}

fn main() {
    let iddan = rdf::IRI::from("http://example.com#iddan");
    let likes = rdf::IRI::from("http://example.com#likes");
    let tamir = rdf::IRI::from("http://example.com#tamir");
    let ontology = rdf::IRI::from("http://example.com#ontology");
    let fact = rdf::Quad {
        subject: iddan,
        predicate: likes,
        object: tamir,
        context: ontology,
    };
    let mut dataset = rdf::Dataset::new();
    dataset.insert(fact);
    println!("{:?}", dataset);
    let fact2 = rdf::Quad {
        subject: tamir,
        predicate: likes,
        object: iddan,
        context: ontology,
    };
    dataset.insert(fact2);
    println!("{:?}", dataset);
    for (subject, predicate) in dataset.subject_predicates(Some(iddan), None) {
        println!("{:?} {:?}", subject, predicate);
    }
    // let mut graph = Graph::new(None);
    // let subject = graph.create_blank_node();
    // let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
    // let object = graph.create_blank_node();
    // let triple = Triple::new(&subject, &predicate, &object);

    // graph.add_triple(&triple);
    // println!("#{:?}", graph);
}
