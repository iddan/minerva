use crate::quad::{Context, Object, Predicate, Quad, Subject};
use crate::store::Store;
use crate::term::{node_to_identifier, Node, IRI};
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MemoryStore<'a> {
    node_to_index: HashMap<&'a Node, NodeIndex>,
    graph: DiGraph<&'a Node, &'a IRI>,
}

impl<'a> MemoryStore<'a> {
    pub fn new() -> MemoryStore<'a> {
        MemoryStore {
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
        let subject_node = self.graph.node_weight(subject_index).unwrap().to_owned();
        let subject = node_to_identifier(subject_node).unwrap();
        let object_index = edge.target();
        let object = self.graph.node_weight(object_index).unwrap().to_owned();
        let predicate = edge.weight().to_owned();
        Quad::new(subject, predicate, object, None)
    }
}

impl<'a> Store<'a> for MemoryStore<'a> {
    fn len(&self) -> usize {
        self.graph.edge_count()
    }

    fn insert_quads(&self, quads: &dyn Iterator<Item = &'a Quad<'a>>) {
        for quad in quads {
            let subject = quad.subject;
            let predicate = quad.predicate;
            let object = quad.object;
            let context = quad.context;
            let subject_index = self.graph.add_node(subject);
            let object_index = self.graph.add_node(object);
            self.node_to_index[subject] = subject_index;
            self.node_to_index[object] = object_index;
            self.graph.add_edge(subject_index, object_index, predicate);
            match context {
                Some(iri) => unimplemented!(),
                None => {}
            }
        }
    }

    fn match_quads(
        &self,
        subject: Option<Subject<'a>>,
        predicate: Option<Predicate<'a>>,
        object: Option<Object<'a>>,
        context: Context<'a>,
    ) -> dyn Iterator<Item = Quad<'a>> {
        match (subject, predicate, object, context) {
            (Some(subject), None, None, None) => {
                let subject_index = self.node_to_index[subject.into()];
                self.graph
                    .edges_directed(subject_index, Direction::Outgoing)
                    .map(|edge| self.edge_to_quad(edge))
            }
            (None, Some(predicate), None, None) => {
                unimplemented!();
            }
            (None, None, Some(object), None) => {
                let node: &'a Node = object.into();
                let object_index = self.node_to_index[object.into()];
                self.graph
                    .edges_directed(object_index, Direction::Incoming)
                    .map(|edge| self.edge_to_quad(edge))
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
