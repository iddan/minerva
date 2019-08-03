FROM rust as builder
ADD . .
RUN cargo build --release

FROM scratch
COPY --from=builder /fs/target/release /
ENTRYPOINT [ "minerva" ]