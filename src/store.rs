use crate::quad::{Context, Object, Predicate, Quad, Subject};
use std::fmt::Debug;

// TODO futures

#[cfg_attr(rustfmt, rustfmt_skip)]
pub trait Quads<'a> = Iterator<Item = Quad<'a>>;

pub trait Store<'a>: Debug + Sync {
    fn match_quads(
        &self,
        subject: Option<Subject<'a>>,
        predicate: Option<Predicate<'a>>,
        object: Option<Object<'a>>,
        context: Context<'a>,
    ) -> Quads<'a>;

    fn len(&self) -> usize;

    fn insert_quads(&self, quads: &Iterator<Item = &'a Quad<'a>>);
}
