use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::quad::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    quads: HashSet<Quad>,
}

impl Dataset {
    pub fn new() -> Dataset {
        Dataset {
            quads: HashSet::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.quads.len()
    }
    pub fn insert(&mut self, quad: Quad) {
        self.quads.insert(quad);
    }
    pub fn contains(&self, quad: &Quad) -> bool {
        return self.quads.contains(quad);
    }
    pub fn match_quads(
        &self,
        subject: Option<Subject>,
        predicate: Option<Predicate>,
        object: Option<Object>,
        context: Option<Context>,
    ) -> impl Iterator<Item = Quad> {
        self.quads.to_owned().into_iter().filter(move |quad| {
            (subject.clone().map_or(true, |v| quad.subject == v)
                && predicate.clone().map_or(true, |v| quad.predicate == v)
                && object.clone().map_or(true, |v| quad.object == v))
                && context.clone().map_or(true, |v| quad.context == v)
        })
    }
    pub fn subjects(
        &self,
        predicate: Option<Predicate>,
        object: Option<Object>,
        context: Option<Context>,
    ) -> impl Iterator<Item = Subject> {
        self.match_quads(None, predicate, object, context)
            .map(|quad| quad.subject)
    }
    pub fn predicates(
        &self,
        subject: Option<Subject>,
        object: Option<Object>,
        context: Option<Context>,
    ) -> impl Iterator<Item = Predicate> {
        self.match_quads(subject, None, object, context)
            .map(|quad| quad.predicate)
    }
    pub fn objects(
        &self,
        subject: Option<Subject>,
        predicate: Option<Predicate>,
        context: Option<Context>,
    ) -> impl Iterator<Item = Object> {
        self.match_quads(subject, predicate, None, context)
            .map(|quad| quad.object)
    }
    pub fn subject_objects(
        &self,
        predicate: Option<Predicate>,
        context: Option<Context>,
    ) -> impl Iterator<Item = (Subject, Object)> {
        self.match_quads(None, predicate, None, context)
            .map(|quad| (quad.subject, quad.object))
    }
    pub fn subject_predicates(
        &self,
        object: Option<Object>,
        context: Option<Context>,
    ) -> impl Iterator<Item = (Subject, Predicate)> {
        self.match_quads(None, None, object, context)
            .map(|quad| (quad.subject, quad.predicate))
    }
    pub fn predicate_objects(
        &self,
        subject: Option<Subject>,
        context: Option<Context>,
    ) -> impl Iterator<Item = (Predicate, Object)> {
        self.match_quads(subject, None, None, context)
            .map(|quad| (quad.predicate, quad.object))
    }
}

impl From<Vec<Quad>> for Dataset {
    fn from(vector: Vec<Quad>) -> Dataset {
        let mut dataset = Dataset::new();
        dataset.extend(vector);
        dataset
    }
}

impl IntoIterator for Dataset {
    type Item = Quad;
    type IntoIter = std::collections::hash_set::IntoIter<Quad>;

    fn into_iter(self) -> Self::IntoIter {
        self.quads.into_iter()
    }
}

impl Add for Dataset {
    type Output = Dataset;

    fn add(self, other: Dataset) -> Dataset {
        Dataset {
            quads: self.quads.union(&other.quads).cloned().collect(),
        }
    }
}

impl AddAssign for Dataset {
    fn add_assign(&mut self, other: Dataset) {
        *self = Dataset {
            quads: self.quads.union(&other.quads).cloned().collect(),
        };
    }
}

impl Sub for Dataset {
    type Output = Dataset;

    fn sub(self, other: Dataset) -> Dataset {
        Dataset {
            quads: self.quads.difference(&other.quads).cloned().collect(),
        }
    }
}

impl SubAssign for Dataset {
    fn sub_assign(&mut self, other: Dataset) {
        *self = Dataset {
            quads: self.quads.difference(&other.quads).cloned().collect(),
        };
    }
}

impl Extend<Quad> for Dataset {
    fn extend<T: IntoIterator<Item = Quad>>(&mut self, iter: T) {
        self.quads.extend(iter)
    }
}
