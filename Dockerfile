FROM rust as builder
ADD . .
RUN cargo build --release

FROM scratch
COPY --from=builder /fs /
ENTRYPOINT [ "target/release/rdf-server" ]