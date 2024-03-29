use crate::term::*;
use serde::{Deserialize, Serialize};

pub type Subject = Identifier;
pub type Predicate = IRI;
pub type Object = Node;
pub type Context = Option<Identifier>;


#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Quad {
    pub subject: Subject,
    pub predicate: Predicate,
    pub object: Object,
    pub context: Context,
}

impl Quad {
    pub fn new<S, P, O, C>(subject: S, predicate: P, object: O, context: C) -> Quad
    where
        S: Into<Subject>,
        P: Into<Predicate>,
        O: Into<Object>,
        C: Into<Context>,
    {
        Quad {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            context: context.into(),
        }
    }
}