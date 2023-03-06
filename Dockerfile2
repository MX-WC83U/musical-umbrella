FROM rust:1.67      
#FROM debian:11-slim
#RUN apt update && apt upgrade -y
#RUN apt install build-essential libssl-dev pkg-config cmake -y

COPY . .
#RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
#ENV PATH="/root/.cargo/bin:$PATH"

RUN export TELOXIDE_TOKEN=<TOKEN>

RUN cargo run --release 

