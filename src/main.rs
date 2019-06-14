mod dataset;
mod quad;
mod term;
use dataset::Dataset;
use quad::Quad;
use term::{Node, IRI};

#[allow(dead_code)]

fn main() {
    let iddan = IRI::from("http://example.com#iddan");
    let likes = IRI::from("http://example.com#likes");
    let tamir = IRI::from("http://example.com#tamir");
    let ontology = IRI::from("http://example.com#ontology");
    let fact = Quad::new(iddan, likes, tamir, ontology);
    let mut dataset = Dataset::new();
    dataset.insert(fact);
    println!("{:?}", dataset);
    let fact2 = Quad::new(tamir, likes, iddan, ontology);
    dataset.insert(fact2);
    println!("{:?}", dataset);
    for (subject, predicate) in dataset.subject_predicates(Some(Node::from(iddan)), None) {
        println!("{:?} {:?}", subject, predicate);
    }
}
