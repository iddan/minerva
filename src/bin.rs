use env_logger;
use futures::future::Future;
use log;
use log::info;
use minerva::dataset::Dataset;
use minerva::namespace::{Namespace, RDF};
use minerva::quad::Quad;
use minerva::server_http;
use minerva::term::{BlankNode, Identifier};

fn main() {
    // Example data
    let example = Namespace::new("http://example.com#");
    let iddan = example.iri("iddan");
    let likes = example.iri("likes");
    let tamir = example.iri("tamir");
    let lior = example.iri("lior");
    let person_type = example.iri("Person");
    let ontology = example.iri("ontology");
    let context = Some(Identifier::IRI(ontology));
    let dataset = Dataset::from(vec![
        Quad::new(&iddan, &likes, &tamir, context.clone()),
        Quad::new(&tamir, &likes, &iddan, context.clone()),
        Quad::new(&iddan, RDF.iri("type"), &person_type, context.clone()),
        Quad::new(&tamir, RDF.iri("type"), &person_type, context.clone()),
        Quad::new(&tamir, &likes, BlankNode::new(), context.clone()),
        Quad::new(lior, RDF.iri("type"), &person_type, context.clone()),
    ]);

    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    let address = "127.0.0.1:31013";

    info!("Listening on {}", address);

    tokio::run(server_http::serve(dataset, address).map_err(|e| eprintln!("server error: {}", e)));
}
