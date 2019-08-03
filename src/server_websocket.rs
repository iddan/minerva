
use std::sync::{Arc,Mutex,MutexGuard};

use futures::{future, Future, Sink, Stream};
use std::error::Error;

use websocket::r#async::Server;
use websocket::message::{OwnedMessage};
use serde_cbor;

use crate::dataset::Dataset;

pub mod messages {
	use serde::{Deserialize, Serialize};
	use crate::quad::{Subject, Predicate, Object, Context};

	#[derive(Serialize, Deserialize, Debug, Default)]
	pub struct Read {
		pub subject: Option<Subject>,
		pub predicate: Option<Predicate>,
		pub object: Option<Object>,
		pub context: Option<Context>,
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub enum IncomingMessage {
		Read(Read)
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub enum OutgoingMessage {
		Ok(())
	}
}

fn handle_connection(addr: std::net::SocketAddr) {
    println!("Got a connection from: {}", addr);
}

fn handle_message(dataset: MutexGuard<Dataset>, message: messages::IncomingMessage) -> messages::OutgoingMessage {
	println!("Message from client {:?}", message);
	match message {
		messages::IncomingMessage::Read(read_message) => {
			println!("{:?}", read_message);
			messages::OutgoingMessage::Ok(())
		}
	}
}

fn serve(dataset: Arc<Mutex<Dataset>>, address: &str) -> impl Future<Item=(), Error=impl Error> {
	// bind to the server
	let server = Server::bind(address, &tokio::reactor::Handle::default()).unwrap();

	println!("Listening on {}", address);

	// a stream of incoming connections
	server
		.incoming()
		.then(future::ok) // wrap good and bad events into future::ok
		.and_then(|event| event) // unwrap good connections
		.map_err(|invalid_connection| invalid_connection.error)
		.and_then(move |(upgrade, addr)| {
			let dataset_ref = Arc::clone(&dataset);
            handle_connection(addr);
			// // check if it has the protocol we want
			// if !upgrade.protocols().iter().any(|s| s == "rust-websocket") {
			// 	// reject it if it doesn't
			// 	spawn_future(upgrade.reject(), "Upgrade Rejection", &executor);
			// 	return Ok(());
			// }

			// accept the request to be a ws connection if it does
			upgrade
				.use_protocol("rust-websocket")
				.accept()
				.and_then(|(s, _)| {
					let (sink, stream) = s.split();
					stream
						.take_while(|m| Ok(!m.is_close()))
						.map(move |m| {
							let d = dataset_ref.lock().unwrap();
							match m {
								OwnedMessage::Binary(data) => {
									let message: messages::IncomingMessage = serde_cbor::from_slice(&data).unwrap();
									let response = handle_message(d, message);
									let serialized_response = serde_cbor::to_vec(&response).unwrap();
									OwnedMessage::Binary(serialized_response)
								},
								_ => m,
							}
						})
						.forward(sink)
						.and_then(|(_, sink)| sink.send(OwnedMessage::Close(None)))
						.and_then(|_| Ok(()))
				})
		})
		.into_future()
}

#[cfg(test)]
mod tests {
	use std::thread;
	use std::sync::{Arc,Mutex};
	use serde_cbor;
	use websocket::message::{Message};
	use websocket::client::ClientBuilder;
    use crate::dataset::Dataset;
	use crate::server;
    #[test]
    fn it_works() {
		let address = "localhost:8889";
		let prefixed = "ws://".to_owned() + address;

		let handler = thread::spawn(move || {
			let dataset = Dataset::new();
			let stream = server::serve(Arc::new(Mutex::new(dataset)), address);
			tokio::run(stream);
		});

		let mut client = ClientBuilder::new(&prefixed)
			.unwrap()
			.connect_insecure()
			.unwrap();
		
		let message = server::messages::IncomingMessage::Read(server::messages::Read{..Default::default()});
		let serialized_message = serde_cbor::to_vec(&message).unwrap();
		
		let result = client.send_message(&Message::binary(serialized_message)).unwrap();
		assert_eq!((), result);
		handler.join().unwrap();
	}
}