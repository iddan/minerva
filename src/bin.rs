use futures::future::Future;
use rdf;

use rdf::dataset::Dataset;
use rdf::namespace::{Namespace, RDF};
use rdf::quad::Quad;
use rdf::term::{BlankNode};
use log;
use log::info;
use env_logger;

fn main() {
    // Example data
    let example = Namespace::new("http://example.com#");
    let iddan = example.iri("iddan");
    let likes = example.iri("likes");
    let tamir = example.iri("tamir");
    let lior = example.iri("lior");
    let person_type = example.iri("Person");
    let ontology = example.iri("ontology");
    let dataset = Dataset::from(vec![
        Quad::new(&iddan, &likes, &tamir, &ontology),
        Quad::new(&tamir, &likes, &iddan, &ontology),
        Quad::new(&iddan, RDF.iri("type"), &person_type, &ontology),
        Quad::new(&tamir, RDF.iri("type"), &person_type, &ontology),
        Quad::new(&tamir, &likes, BlankNode::new(None), &ontology),
        Quad::new(lior, RDF.iri("type"), &person_type, &ontology),
    ]);

    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    let address = "127.0.0.1:4567";

    info!("Listening on {}", address);

    tokio::run(
        rdf::server_http::serve(dataset, address)
            .map_err(|e| eprintln!("server error: {}", e))
    );
}