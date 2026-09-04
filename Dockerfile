# syntax=docker/dockerfile:1

# ---- builder ----
FROM rust:1-slim-bookworm AS builder

ARG HTTP_PROXY
ARG HTTPS_PROXY
ARG ALL_PROXY

ENV HTTP_PROXY=${HTTP_PROXY}
ENV HTTPS_PROXY=${HTTPS_PROXY}
ENV ALL_PROXY=${ALL_PROXY}

ENV http_proxy=${HTTP_PROXY}
ENV https_proxy=${HTTPS_PROXY}
ENV all_proxy=${ALL_PROXY}

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# ---- runtime ----
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/bdtv_hub /usr/local/bin/

WORKDIR /app

EXPOSE 7497

CMD ["bdtv_hub"]