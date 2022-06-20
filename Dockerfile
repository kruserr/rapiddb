FROM docker.io/rust:1.61
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
CMD /usr/src/app/target/release/rapiddb
