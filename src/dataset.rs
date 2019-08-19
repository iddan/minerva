// use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::quad::*;

pub trait Dataset<'a>: Extend<Quad<'a>> {
    fn len(&self) -> usize;

    fn match_quads(
        &self,
        subject: Option<Subject<'a>>,
        predicate: Option<Predicate<'a>>,
        object: Option<Object<'a>>,
        context: Context<'a>,
    ) -> Box<dyn Iterator<Item = Quad<'a>> + 'a>;

    fn insert(&mut self, quad: Quad<'a>) {
        self.extend(vec![quad]);
    }

    fn contains(&self, quad: Quad<'a>) -> bool {
        unimplemented!()
    }
    // pub fn subjects(
    //     &self,
    //     predicate: Option<Predicate>,
    //     object: Option<Object>,
    //     context: Context,
    // ) -> impl Iterator<Item = Subject<'a>> {
    //     self.store
    //         .match_quads(None, predicate, object, context)
    //         .map(|quad| quad.subject)
    // }
    // pub fn predicates(
    //     &self,
    //     subject: Option<Subject>,
    //     object: Option<Object>,
    //     context: Context,
    // ) -> impl Iterator<Item = Predicate<'a>> {
    //     self.store
    //         .match_quads(subject, None, object, context)
    //         .map(|quad| quad.predicate)
    // }
    // pub fn objects(
    //     &self,
    //     subject: Option<Subject>,
    //     predicate: Option<Predicate>,
    //     context: Context,
    // ) -> impl Iterator<Item = Object<'a>> {
    //     self.store
    //         .match_quads(subject, predicate, None, context)
    //         .map(|quad| quad.object)
    // }
    // pub fn subject_objects(
    //     &self,
    //     predicate: Option<Predicate>,
    //     context: Context,
    // ) -> impl Iterator<Item = (Subject<'a>, Object<'a>)> {
    //     self.store
    //         .match_quads(None, predicate, None, context)
    //         .map(|quad| (quad.subject, quad.object))
    // }
    // pub fn subject_predicates(
    //     &self,
    //     object: Option<Object>,
    //     context: Context,
    // ) -> impl Iterator<Item = (Subject<'a>, Predicate<'a>)> {
    //     self.store
    //         .match_quads(None, None, object, context)
    //         .map(|quad| (quad.subject, quad.predicate))
    // }
    // pub fn predicate_objects(
    //     &self,
    //     subject: Option<Subject>,
    //     context: Context,
    // ) -> impl Iterator<Item = (Predicate<'a>, Object<'a>)> {
    //     self.store
    //         .match_quads(subject, None, None, context)
    //         .map(|quad| (quad.predicate, quad.object))
    // }
}

// impl<'a> Add for Dataset<'a> {
//     type Output = Dataset<'a>;

//     fn add(self, other: Dataset<'a>) -> Dataset<'a> {
//         Dataset {
//             quads: self.quads.union(&other.quads).cloned().collect(),
//         }
//     }
// }

// impl AddAssign for Dataset {
//     fn add_assign(&mut self, other: Dataset) {
//         *self = Dataset {
//             quads: self.quads.union(&other.quads).cloned().collect(),
//         };
//     }
// }

// impl Sub for Dataset {
//     type Output = Dataset;

//     fn sub(self, other: Dataset) -> Dataset {
//         Dataset {
//             quads: self.quads.difference(&other.quads).cloned().collect(),
//         }
//     }
// }

// impl SubAssign for Dataset {
//     fn sub_assign(&mut self, other: Dataset) {
//         *self = Dataset {
//             quads: self.quads.difference(&other.quads).cloned().collect(),
//         };
//     }
// }