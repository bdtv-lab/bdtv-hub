# ---- builder ----
FROM rust:1-slim-bookworm AS builder

# reqwest 默认走 rustls + aws-lc-rs，后者编译需要 cmake
RUN apt-get update \
    && apt-get install -y --no-install-recommends cmake \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# ---- runtime ----
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/servers-management-center /usr/local/bin/

# load_config() 按相对路径读写 config.yaml，所以工作目录要挂出去
WORKDIR /app

EXPOSE 7497

CMD ["servers-management-center"]
