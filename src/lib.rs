#![crate_type = "lib"]
#![crate_name = "minerva"]
#![feature(try_blocks)]

pub mod dataset;
pub mod namespace;
pub mod quad;
pub mod server_http;
// mod server_websocket;
pub mod term;
pub mod nquads_serialize;
pub mod nquads_deserialize;
mod read_service;

#[cfg(test)]
mod tests {
    use crate::dataset::Dataset;
    use crate::namespace::{Namespace, RDF};
    use crate::quad::Quad;
    use crate::term::{Node, IRI, Identifier};
    #[test]
    fn it_works() {
        let example = Namespace::new("http://example.com#");
        let iddan = example.iri("iddan");
        let likes = example.iri("likes");
        let tamir = example.iri("tamir");
        let Person = example.iri("Person");
        let ontology = example.iri("ontology");
        let context = Some(Identifier::IRI(ontology));
        let fact = Quad::new(&iddan, &likes, &tamir, context.clone());
        let mut dataset = Dataset::new();
        dataset.insert(fact);
        println!("{:?}", dataset);
        let fact2 = Quad::new(&tamir, &likes, &iddan, context.clone());
        dataset.insert(fact2);
        println!("{:?}", dataset);
        for (subject, predicate) in dataset.subject_predicates(Some(Node::from(&iddan)), None) {
            println!("{:?} {:?}", subject, predicate);
        }
        let fact3 = Quad::new(&iddan, RDF.iri("type"), &Person, context.clone());
        let fact4 = Quad::new(&tamir, RDF.iri("type"), &Person, context.clone());
        let lior = IRI::new("http://example.com/test#lior");
        let fact5 = Quad::new(lior, RDF.iri("type"), &Person, context.clone());
        dataset.extend(vec![fact3, fact4, fact5]);
        println!("{:?}", serde_cbor::to_vec(&dataset));
    }
}
