#![crate_type = "lib"]
#![crate_name = "minerva"]
#![feature(try_blocks)]
#![feature(trait_alias)]

pub mod dataset;
pub mod memory_store;
pub mod namespace;
mod no_error;
pub mod nquads_deserialize;
pub mod nquads_serialize;
pub mod quad;
mod read_service;
pub mod server_http;
pub mod store;
pub mod term;
mod test_set;
mod write_service;

#[cfg(test)]
mod tests {
    use crate::dataset::Dataset;
    use crate::memory_store::MemoryStore;
    use crate::namespace::{Namespace, RDF};
    use crate::quad::Quad;
    use crate::term::{Identifier, Node, IRI};
    #[test]
    fn it_works() {
        let example = Namespace::new("http://example.com#");
        let iddan = example.iri("iddan");
        let likes = example.iri("likes");
        let tamir = example.iri("tamir");
        let _type = RDF.iri("type");
        let Person = example.iri("Person");
        let ontology = example.iri("ontology");
        let context = Some(&Identifier::IRI(ontology));
        let fact = Quad::new(&iddan, &likes, &tamir, context);
        let mut store = MemoryStore::new();
        let mut dataset = Dataset::new(&store);
        dataset.insert(fact);
        println!("{:?}", dataset);
        let fact2 = Quad::new(&tamir, &likes, &iddan, context);
        dataset.insert(fact2);
        println!("{:?}", dataset);
        // for (subject, predicate) in dataset.subject_predicates(Some(Node::from(iddan)), None) {
        //     println!("{:?} {:?}", subject, predicate);
        // }
        let fact3 = Quad::new(&iddan, &_type, &Person, context);
        let fact4 = Quad::new(&tamir, &_type, &Person, context);
        let lior = IRI::new("http://example.com/test#lior");
        let fact5 = Quad::new(&lior, &_type, &Person, context);
        dataset.extend(vec![fact3, fact4, fact5]);
    }
}
