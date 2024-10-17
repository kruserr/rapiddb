FROM docker.io/rust:1.82-slim-bookworm AS builder
RUN apt-get update && \
  apt-get install -y pkg-config libssl-dev && \
  rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM docker.io/debian:bookworm-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/rapiddb-web ./
CMD /usr/src/app/rapiddb-web
