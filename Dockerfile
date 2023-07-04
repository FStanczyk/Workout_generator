FROM rust:latest as build

WORKDIR /workout-generator
COPY . .

RUN cargo install --path .

CMD ["workout-generator"]
