use crate::dataset::Dataset;
use crate::no_error::NoError;
use crate::quad::{Context, Object, Predicate, Quad, Subject};
use futures::stream;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Params<'a> {
    pub subject: Option<Subject<'a>>,
    pub predicate: Option<Predicate<'a>>,
    pub object: Option<Object<'a>>,
    pub context: Context<'a>,
}

pub fn read<'a>(
    params: Params<'a>,
    dataset_lock: &'a Mutex<Dataset<'a>>,
) -> impl stream::Stream<Item = Quad<'a>, Error = NoError> {
    let dataset = dataset_lock.lock().unwrap();
    return stream::iter_ok::<_, NoError>(dataset.match_quads(
        params.subject,
        params.predicate,
        params.object,
        params.context,
    ));
}
