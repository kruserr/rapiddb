FROM docker.io/rust:1.61-slim-bullseye AS builder
RUN apt-get update && \
  apt-get install -y pkg-config libssl-dev && \
  rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM docker.io/debian:bullseye-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/rapiddb ./
CMD /usr/src/app/rapiddb
