FROM rust:1.67.0

COPY . .

ENV TELOXIDE_TOKEN=<TOKEN>

RUN cargo build --release

CMD ["cargo", "run", "--release"]
