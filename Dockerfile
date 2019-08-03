FROM rust
ADD . .
RUN cargo build --release
CMD target/release/rdf-server