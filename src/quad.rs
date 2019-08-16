use crate::term::*;

pub type Subject<'a> = &'a Identifier;
pub type Predicate<'a> = &'a IRI;
pub type Object<'a> = &'a Node;
pub type Context<'a> = Option<&'a Identifier>;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Quad<'a> {
    pub subject: Subject<'a>,
    pub predicate: Predicate<'a>,
    pub object: Object<'a>,
    pub context: Context<'a>,
}

impl<'a> Quad<'a> {
    pub fn new<S, P, O, C>(subject: S, predicate: P, object: O, context: C) -> Quad<'a>
    where
        S: Into<Subject<'a>>,
        P: Into<Predicate<'a>>,
        O: Into<Object<'a>>,
        C: Into<Context<'a>>,
    {
        Quad {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            context: context.into(),
        }
    }
}
