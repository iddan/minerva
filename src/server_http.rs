use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use std::sync::{Arc,Mutex};
use serde::{Deserialize, Serialize};
use serde_qs;
use crate::dataset::Dataset;
use crate::quad::{Subject, Predicate, Object, Context};
use crate::nquads_serialize;
use log::{info};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ReadParams {
    pub subject: Option<Subject>,
    pub predicate: Option<Predicate>,
    pub object: Option<Object>,
    pub context: Option<Context>,
}

impl From<Request<Body>> for ReadParams {
    fn from(request: Request<Body>) -> ReadParams {
        match request.uri().query() {
            Some(query) => {
                serde_qs::from_str(&query).unwrap()
            },
            None => {
                ReadParams {
                    subject: None,
                    predicate: None,
                    object: None,
                    context: None
                }
            }
        }
    }
}

fn read_quads(request: Request<Body>, dataset_lock: &Mutex<Dataset>) -> Response<Body> {
    if request.uri().path() == "/" {
        info!("{} {}", request.method(), request.uri().to_string());
        let params: ReadParams = request.into();
        let dataset = dataset_lock.lock().unwrap();
        let quads = dataset.match_quads(params.subject, params.predicate, params.object, params.context);
        let stream = futures::stream::iter_ok::<_, hyper::Error>(nquads_serialize::serialize_quad_iterator(quads));
        return Response::builder()
            .status(200)
            .header("Content-Type", "x-nquads")
            .body(Body::wrap_stream(stream))
            .unwrap()
    }
    Response::builder()
        .status(404)
        .body(Body::from(""))
        .unwrap()
}

pub fn serve(dataset: Dataset, address: &str) -> impl Future<Item=(), Error=hyper::Error> {
    // This is our socket address...
    let socket_address = address.parse().unwrap();
    let shared_dataset = Arc::new(Mutex::new(dataset));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let service = move || {
        let cloned_dataset = Arc::clone(&shared_dataset);
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(move |request| read_quads(request, &cloned_dataset))
    };

    Server::bind(&socket_address)
        .serve(service)
}