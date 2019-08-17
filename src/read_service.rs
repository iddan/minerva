use serde::{Serialize, Deserialize};
use crate::dataset::Dataset;
use crate::memory_dataset::MemoryDataset;
use crate::nquads_deserialize;
use crate::quad::{Quad};
use crate::no_error::NoError;
use futures::stream;
use std::sync::Mutex;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Params {
    pub subject: Option<String>,
    pub predicate: Option<String>,
    pub object: Option<String>,
    pub context: Option<String>,
}

pub fn read<'a>(
    params: Params,
    dataset_lock: &'a Mutex<MemoryDataset<'a>>,
) -> Result<impl stream::Stream<Item = Quad<'a>, Error = NoError>, String> {
    let dataset = dataset_lock.lock().unwrap();
    let subject = match params.subject {
        Some(value) => Some(&nquads_deserialize::deserialize_identifier(&mut value.chars().peekable())?),
        None => None
    };
    let predicate = match params.predicate {
        Some(value) => Some(&nquads_deserialize::deserialize_iri(&mut value.chars().peekable())?),
        None => None
    };
    let object = match params.object {
        Some(value) => Some(&nquads_deserialize::deserialize_node(&mut value.chars().peekable())?),
        None => None
    };
    let context = match params.context {
        Some(value) => Some(&nquads_deserialize::deserialize_identifier(&mut value.chars().peekable())?),
        None => None
    };
    return Ok(stream::iter_ok::<_, NoError>(dataset.match_quads(
        subject,
        predicate,
        object,
        context,
    )));
}
