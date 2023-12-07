FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
WORKDIR /rust-brag-server
COPY . .
RUN cargo build --release --bin load_db --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /rust-brag-server/target/x86_64-unknown-linux-musl/release/load_db /rust-brag-server
ENTRYPOINT ["/load_db"]
