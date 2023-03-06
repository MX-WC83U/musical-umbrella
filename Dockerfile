FROM rust:latest as build 
RUN cargo install teloxide && \
  cargo install tokio && \
  cargo install passwords && \
  cargo install log && \
  cargo install pretty_env_logger


WORKDIR /root/musical-umbrella/src

COPY . .

RUN cd musical-umbrella && cargo build --release 


FROM debian:11-slim

COPY --from=build /root/musical-umbrella/src/target/release
 /usr/local/bin

WORKDIR /usr/local/bin
CMD ["./main"]
