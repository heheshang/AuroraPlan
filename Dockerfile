FROM rust:latest as builder
USER root
ENV TZ=Asia/Shanghai

COPY . /AuroraPlan
WORKDIR /AuroraPlan
RUN mkdir -p /$HOME/.cargo

RUN touch /$HOME/.cargo/config.toml
COPY .cargo/config.toml  /$HOME/.cargo/config.toml
RUN rm -rf /AuroraPlan/aurora-proto/build.rs 
RUN ls -l /AuroraPlan/aurora-proto

ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
ENV PATH="/root/.cargo/bin:$PATH"
RUN sed -i 's/rustc-wrapper = "sccache"/#rustc-wrapper = "sccache"/g'  /$HOME/.cargo/config.toml
RUN sed -i 's/rustc-wrapper = "sccache"/#rustc-wrapper = "sccache"/g'  .cargo/config.toml
RUN cargo install cargo-nextest
RUN cargo nextest run -F cn_msg --nocapture --release
RUN cargo build --release

RUN mkdir -p /AuroraPlan/deploy/examples
RUN mkdir -p /AuroraPlan/ui
RUN ls -la /AuroraPlan/target/release
RUN cp /AuroraPlan/target/release/aurora-api /AuroraPlan/deploy/
RUN cp /AuroraPlan/target/release/aurora-service /AuroraPlan/deploy/
RUN cp /AuroraPlan/target/release/migration /AuroraPlan/deploy/
RUN cp /AuroraPlan/target/release/aurora-tests-* /AuroraPlan/deploy/
RUN cp -r /AuroraPlan/target/release/examples /AuroraPlan/deploy/examples/
RUN ls -la /AuroraPlan/deploy/
RUN ls -la /AuroraPlan/deploy/examples
RUN cargo clean
RUN rm -rf /root/.cargo/registry/cache

FROM node:latest as node
USER root
ENV TZ=Asia/Shanghai
WORKDIR /AuroraPlan
COPY --from=builder /AuroraPlan .
RUN cd /AuroraPlan/aurora-ui && npm install && npm run build:prod
RUN ls -la /AuroraPlan/aurora-ui/dist
RUN cp -r /AuroraPlan/aurora-ui/dist/* /AuroraPlan/ui


FROM rust:latest
USER root
ENV TZ=Asia/Shanghai
WORKDIR /AuroraPlan
COPY --from=builder /AuroraPlan .
COPY --from=node /AuroraPlan/ui /AuroraPlan/ui
RUN ls -la /AuroraPlan/deploy/
RUN ls -la /AuroraPlan/deploy/examples
RUN ls -la /AuroraPlan/ui

EXPOSE 8000

