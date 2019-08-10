use futures::future;
use std::sync::{Arc,Mutex};
use futures::stream::Stream;
use http::method::Method;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::{make_service_fn, service_fn};
use log::{info};
use serde_qs;
use crate::dataset::Dataset;
use crate::nquads_serialize;
use crate::nquads_deserialize;
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

fn quads_service_get<'a>(request: Request<Body>, dataset_lock: Arc<Mutex<Dataset>>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    let params: read_service::Params = request.into();
    let quads = read_service::read(params, &dataset_lock);
    let stream = futures::stream::iter_ok::<_, hyper::Error>(nquads_serialize::serialize(quads));
    Box::new(future::ok(Response::builder()
        .status(200)
        .header("Content-Type", "x-nquads")
        .body(Body::wrap_stream(stream))
        .unwrap()))
}


fn quads_service_post(request: Request<Body>, dataset_lock: Arc<Mutex<Dataset>>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    Box::new(request.into_body().concat2().and_then(move |body| {
        // TODO error handle
        let nquads = String::from_utf8(body.to_vec()).unwrap();
        let quads = nquads_deserialize::deserialize(&nquads);
        // TODO move
        let mut dataset = dataset_lock.lock().unwrap();
        for result in quads {
            if result.is_err() {
                return Ok(
                    Response::builder()
                        .status(401)
                        .body(Body::empty())
                        .unwrap()
                )
            }
            dataset.insert(result.unwrap())
        }
        Ok(Response::builder()
            .status(201)
            .body(Body::empty())
            .unwrap())
    }))
}


fn quad_service_unknown_method() -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    Box::new(future::ok(Response::builder()
        .status(405)
        .body(Body::empty())
        .unwrap()))
}


fn quad_service_unknown_path() -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    Box::new(future::ok(Response::builder()
                .status(404)
                .body(Body::empty())
                .unwrap()))
}


pub fn serve(dataset: Dataset, address: &str) -> impl Future<Item=(), Error=hyper::Error> {
    let socket_address = address.parse().unwrap();
    let shared_dataset = Arc::new(Mutex::new(dataset));
    let make_service = make_service_fn(move |_| {
        let cloned_dataset = Arc::clone(&shared_dataset);
        service_fn(move |request| {
            let cloned_dataset = Arc::clone(&cloned_dataset);
            let method = request.method();
            let uri = request.uri();
            info!("{} {}", method, uri.to_string());
            let path = uri.path();
            match (method, path) {
                (&Method::GET, "/") => quads_service_get(request, cloned_dataset),
                (&Method::POST, "/") => quads_service_post(request, cloned_dataset),
                (_, "/") => quad_service_unknown_method(),
                _ => quad_service_unknown_path()
            }
        })
    });

    Server::bind(&socket_address)
        .serve(make_service)
}