use futures::future::Future;
use rdf;

use rdf::dataset::Dataset;
use rdf::namespace::{Namespace, RDF};
use rdf::quad::Quad;
use rdf::term::{Node, IRI};
use log;
use log::info;
use env_logger;

fn main() {
    let dataset = rdf::dataset::Dataset::new();

    // Example data
    let example = Namespace::new("http://example.com#");
    let iddan = example.iri("iddan");
    let likes = example.iri("likes");
    let tamir = example.iri("tamir");
    let Person = example.iri("Person");
    let ontology = example.iri("ontology");
    let fact = Quad::new(&iddan, &likes, &tamir, &ontology);
    let mut dataset = Dataset::new();
    let fact2 = Quad::new(&tamir, &likes, &iddan, &ontology);
    let fact3 = Quad::new(&iddan, RDF.iri("type"), &Person, &ontology);
    let fact4 = Quad::new(&tamir, RDF.iri("type"), &Person, &ontology);
    let lior = IRI::new("http://example.com/test#lior");
    let fact5 = Quad::new(lior, RDF.iri("type"), &Person, &ontology);
    dataset.extend(vec![fact, fact2, fact3, fact4, fact5]);

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