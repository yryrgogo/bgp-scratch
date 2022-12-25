FROM rust:1-buster

WORKDIR /bgp-scratch
COPY . .
RUN rustup default nightly && \
  cargo build
CMD ["./target/debug/bgp-scratch", "64512 10.200.100.2 64513 10.200.100.3 active"]
