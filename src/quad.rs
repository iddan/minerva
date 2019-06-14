use crate::term::*;

pub type Subject = Identifier;
pub type Predicate = IRI;
pub type Object = Node;
pub type Context = Identifier;

// For simplicity sake let's keep it IRI triple until I'll understand how to type Nodes correctly.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Quad {
    pub subject: Subject,
    pub predicate: Predicate,
    pub object: Object,
    pub context: Context,
}

impl Quad {
    pub fn new<S, O, C>(subject: S, predicate: IRI, object: O, context: C) -> Quad
    where
        S: Into<Subject>,
        O: Into<Object>,
        C: Into<Context>,
    {
        Quad {
            subject: subject.into(),
            predicate,
            object: object.into(),
            context: context.into(),
        }
    }
}
