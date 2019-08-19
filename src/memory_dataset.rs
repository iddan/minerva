use crate::quad::{Context, Object, Predicate, Quad, Subject};
use crate::dataset::Dataset;
use crate::term::{node_to_identifier, Node, IRI};
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::{Direction};
use std::collections::HashMap;

#[derive(Debug)]
pub struct MemoryDataset<'a> {
    node_to_index: HashMap<&'a Node, NodeIndex>,
    graph: DiGraph<&'a Node, &'a IRI>,
}

impl <'a> MemoryDataset<'a> {
    pub fn new() -> MemoryDataset<'a> {
        MemoryDataset {
            graph: DiGraph::new(),
            node_to_index: HashMap::new(),
        }
    }

    fn edge_to_quad(
        &self,
        edge: impl EdgeRef<NodeId = NodeIndex, EdgeId = EdgeIndex, Weight = &'a IRI>,
    ) -> Quad<'a> {
        // TODO fix to_owned()
        let subject_index = edge.source();
        let subject_node = self.graph[subject_index];
        let subject = node_to_identifier(&subject_node).unwrap();
        let object_index = edge.target();
        let object = self.graph[object_index];
        let predicate = self.graph[edge.id()];
        Quad::new(subject, predicate, object, None)
    }
}

impl <'a> Dataset<'a> for MemoryDataset<'a> {
    fn len(&self) -> usize {
        self.graph.edge_count()
    }

    fn match_quads(
        &self,
        subject: Option<Subject<'a>>,
        predicate: Option<Predicate<'a>>,
        object: Option<Object<'a>>,
        context: Context<'a>,
    ) -> Box<dyn Iterator<Item = Quad<'a>> + 'a> {
        match (subject, predicate, object, context) {
            (Some(subject), None, None, None) => {
                let subject_node: &Node = subject.into();
                let subject_index = self.node_to_index[subject_node];
                let edges = self.graph.edges_directed(subject_index, Direction::Outgoing);
                let quads = edges.map(|edge| self.edge_to_quad(edge));
                Box::new(quads)
            }
            (None, Some(predicate), None, None) => {
                unimplemented!();
            }
            (None, None, Some(object), None) => {
                let node: &'a Node = object.into();
                let object_index = self.node_to_index[object.into()];
                let edges = self.graph.edges_directed(object_index, Direction::Incoming);
                let quads = edges.map(|edge| self.edge_to_quad(edge));
                Box::new(quads)
            }
            (Some(subject), Some(predicate), None, None) => {
                unimplemented!();
            }
            (Some(subject), None, Some(object), None) => {
                // Waiting for https://github.com/bluss/petgraph/pull/237
                unimplemented!();
            }
            (None, Some(predicate), Some(object), None) => {
                unimplemented!();
            }
        }
    }
}

impl <'a>Extend<Quad<'a>> for MemoryDataset<'a> {
    fn extend<T: IntoIterator<Item=Quad<'a>>>(&mut self, quads: T) {
        for quad in quads {
            let subject = quad.subject;
            let predicate = quad.predicate;
            let object = quad.object;
            let context = quad.context;
            let subject_node = subject.into();
            let subject_index = self.graph.add_node(subject_node);
            let object_index = self.graph.add_node(object);
            self.node_to_index[subject_node] = subject_index;
            self.node_to_index[object] = object_index;
            self.graph.add_edge(subject_index, object_index, predicate);
            match context {
                Some(iri) => unimplemented!(),
                None => {}
            }
        }
    }
}