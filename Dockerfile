#FROM rust:1.67      
FROM debian:11-slim
#FROM alpine:latest
#RUN apk add git curl
RUN apt update && apt upgrade -y
RUN apt install build-essential libssl-dev pkg-config cmake git curl -y

WORKDIR /um
#RUN apt update && apt upgrade -y
#RUN apt install build-essential libssl-dev pkg-config cmake -y
RUN git clone https://github.com/broomshed/musical-umbrella
#COPY . .
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:$PATH"

RUN export TELOXIDE_TOKEN=<TOKEN>

CMD ["cargo", "run", "--release"]

