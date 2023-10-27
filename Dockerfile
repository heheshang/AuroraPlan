FROM rust:latest 

COPY . /app
WORKDIR /app

RUN cargo build --release

EXPOSE 8000
