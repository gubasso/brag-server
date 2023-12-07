FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
WORKDIR /rust-brag-server
COPY . .
RUN cargo build --release --bin brag-server --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /rust-brag-server/target/x86_64-unknown-linux-musl/release/brag-server /rust-brag-server
ENTRYPOINT ["/brag-server"]
EXPOSE 3000
