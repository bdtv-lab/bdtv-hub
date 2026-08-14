# syntax=docker/dockerfile:1

# ---- builder ----
FROM rust:1-slim-bookworm AS builder

# apt 换中科大源；只替换主机名，保留原 http 协议
RUN for f in /etc/apt/sources.list /etc/apt/sources.list.d/debian.sources; do \
        if [ -f "$f" ]; then \
            sed -i 's|deb.debian.org|mirrors.ustc.edu.cn|g; s|security.debian.org|mirrors.ustc.edu.cn|g' "$f"; \
        fi; \
    done

# reqwest 默认走 rustls + aws-lc-rs，后者编译需要 cmake
RUN apt-get update \
    && apt-get install -y --no-install-recommends cmake \
    && rm -rf /var/lib/apt/lists/*

# cargo 换中科大源（sparse 协议）
RUN cat > $CARGO_HOME/config.toml <<'EOF'
[source.crates-io]
replace-with = "ustc"

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
EOF

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# ---- runtime ----
FROM debian:bookworm-slim

RUN for f in /etc/apt/sources.list /etc/apt/sources.list.d/debian.sources; do \
        if [ -f "$f" ]; then \
            sed -i 's|deb.debian.org|mirrors.ustc.edu.cn|g; s|security.debian.org|mirrors.ustc.edu.cn|g' "$f"; \
        fi; \
    done

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/servers-management-center /usr/local/bin/

# load_config() 按相对路径读写 config.yaml，所以工作目录要挂出去
WORKDIR /app

EXPOSE 7497

CMD ["servers-management-center"]
