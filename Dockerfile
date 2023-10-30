FROM rust:latest 

COPY . /app
WORKDIR /app
# install protobuf

RUN apt-get update && \
    apt-get install -y protobuf-compiler && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
# Install  Install protoc
RUN cargo install protobuf-codegen
RUN cargo build --release

EXPOSE 8000
