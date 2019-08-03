use std::sync::{Arc,Mutex};
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use log::{info};
use serde_qs;
use crate::dataset::Dataset;
use crate::nquads_serialize;
use crate::read_service;

impl From<Request<Body>> for read_service::Params {
    fn from(request: Request<Body>) -> read_service::Params {
        match request.uri().query() {
            Some(query) => {
                serde_qs::from_str(&query).unwrap()
            },
            None => {
                read_service::Params {
                    subject: None,
                    predicate: None,
                    object: None,
                    context: None
                }
            }
        }
    }
}

fn read_service(request: Request<Body>, dataset_lock: &Mutex<Dataset>) -> Response<Body> {
    if request.uri().path() == "/" {
        info!("{} {}", request.method(), request.uri().to_string());
        let params: read_service::Params = request.into();
        let quads = read_service::read(params, dataset_lock);
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
        service_fn_ok(move |request| read_service(request, &cloned_dataset))
    };

    Server::bind(&socket_address)
        .serve(service)
}