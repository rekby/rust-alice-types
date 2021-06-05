FROM rust AS builder
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /src
COPY dummy.rs .
COPY Cargo.toml .
COPY Cargo.lock .
#COPY .cargo/vendor_config .cargo/config
#COPY vendor vendor
RUN sed -i 's|path="src/main.rs"|path="dummy.rs"|' Cargo.toml
RUN cargo build --target x86_64-unknown-linux-musl --release
#RUN sed -i 's|path="dummy.rs"|path="src/main.rs"|' Cargo.toml
COPY Cargo.toml .
COPY src src
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/alice-storage-life /usr/local/bin/alice-storage-life
CMD ["alice-storage-life"]
