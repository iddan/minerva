use crate::memory_dataset::MemoryDataset;
use crate::nquads_serialize;
use crate::read_service;
use crate::write_service;
use futures::future;
use futures::stream::Stream;
use http::method::Method;
use hyper::rt::Future;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::info;
use serde_qs;
use std::sync::{Arc, Mutex};

impl<'a> From<Request<Body>> for read_service::Params {
    fn from(request: Request<Body>) -> read_service::Params {
        match request.uri().query() {
            Some(query) => serde_qs::from_str(&query).unwrap(),
            None => read_service::Params {
                subject: None,
                predicate: None,
                object: None,
                context: None,
            },
        }
    }
}

fn quads_service_get<'a>(
    request: Request<Body>,
    dataset_lock: Arc<Mutex<MemoryDataset<'a>>>,
) -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send + 'a> {
    let params: read_service::Params = request.into();
    match read_service::read(params, &dataset_lock) {
        Ok(quads) => {
            let stream = nquads_serialize::serialize(quads);
            Box::new(future::ok(
                Response::builder()
                    .status(200)
                    .header("Content-Type", "x-nquads")
                    .body(Body::wrap_stream(stream))
                    .unwrap(),
            ))
        },
        Err(error) => {
            Box::new(future::ok(
                Response::builder()
                    .status(400)
                    .header("Content-Type", "text")
                    .body(Body::from(error))
                    .unwrap()
            ))
        }
    }
}

fn quads_service_post<'a>(
    request: Request<Body>,
    dataset_lock: Arc<Mutex<MemoryDataset<'a>>>,
) -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send + 'a> {
    Box::new(request.into_body().concat2().and_then(move |body| {
        // TODO error handle
        let nquads = String::from_utf8(body.to_vec()).unwrap();
        let result = write_service::write(nquads, &dataset_lock);
        match result {
            Ok(_) => Ok(Response::builder().status(201).body(Body::empty()).unwrap()),
            Err(_) => Ok(Response::builder().status(401).body(Body::empty()).unwrap()),
        }
    }))
}

fn quad_service_unknown_method(
) -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
    Box::new(future::ok(
        Response::builder().status(405).body(Body::empty()).unwrap(),
    ))
}

fn quad_service_unknown_path() -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>
{
    Box::new(future::ok(
        Response::builder().status(404).body(Body::empty()).unwrap(),
    ))
}

pub fn serve<'a>(
    dataset: MemoryDataset<'a>,
    address: &str,
) -> Box<dyn Future<Item = (), Error = hyper::Error> + Send> {
    let socket_address = address.parse().unwrap();
    let shared_dataset: Arc<Mutex<MemoryDataset<'a>>> = Arc::new(Mutex::new(dataset));
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
                _ => quad_service_unknown_path(),
            }
        })
    });

    Box::new(Server::bind(&socket_address).serve(make_service))
}
