use crate::term::*;

pub struct Namespace<A>
where
    A: Into<String>,
{
    address: A,
}

impl<A> Namespace<A>
where
    A: Into<String>,
{
    pub fn new(address: A) -> Namespace<A>
    where
        A: Into<String>,
    {
        Namespace { address: address }
    }

    pub fn iri<N>(&self, name: N) -> IRI
    where
        A: Into<String> + Clone,
        N: Into<String>,
    {
        let address = self.address.clone().into();
        let value = address + &name.into();
        IRI::new(value)
    }
}

pub static RDF: Namespace<&'static str> = Namespace {
    address: "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
};
pub static RDFS: Namespace<&'static str> = Namespace {
    address: "http://www.w3.org/2000/01/rdf-schema#",
};
pub static OWL: Namespace<&'static str> = Namespace {
    address: "http://www.w3.org/2002/07/owl#",
};
pub static XSD: Namespace<&'static str> = Namespace {
    address: "http://www.w3.org/2001/XMLSchema#",
};
pub static SKOS: Namespace<&'static str> = Namespace {
    address: "http://www.w3.org/2004/02/skos/core#",
};
pub static DOAP: Namespace<&'static str> = Namespace {
    address: "http://usefulinc.com/ns/doap#",
};
pub static FOAF: Namespace<&'static str> = Namespace {
    address: "http://xmlns.com/foaf/0.1/",
};
pub static DC: Namespace<&'static str> = Namespace {
    address: "http://purl.org/dc/elements/1.1/",
};
pub static DCTERMS: Namespace<&'static str> = Namespace {
    address: "http://purl.org/dc/terms/",
};
pub static VOID: Namespace<&'static str> = Namespace {
    address: "http://rdfs.org/ns/void#",
};
