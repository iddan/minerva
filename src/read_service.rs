use std::sync::{Mutex};
use serde::{Deserialize, Serialize};
use crate::quad::{Quad, Subject, Predicate, Object, Context};
use crate::dataset::Dataset;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Params {
    pub subject: Option<Subject>,
    pub predicate: Option<Predicate>,
    pub object: Option<Object>,
    pub context: Option<Context>,
}


pub fn read(params: Params, dataset_lock: &Mutex<Dataset>) -> impl Iterator<Item=Quad> {
    let dataset = dataset_lock.lock().unwrap();
    return dataset.match_quads(params.subject, params.predicate, params.object, params.context);
}