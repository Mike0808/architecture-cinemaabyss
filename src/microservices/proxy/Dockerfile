# –– Builder Stage ––

FROM rust:1.84-slim-bullseye as builder

# 1. Configure TUNA mirrors for both Debian and Rust

# RUN echo 'deb https://mirrors.tuna.tsinghua.edu.cn/debian/ bullseye main contrib non-free' > /etc/apt/sources.list &&   \
#     echo 'deb https://mirrors.tuna.tsinghua.edu.cn/debian/ bullseye-updates main contrib non-free' >> /etc/apt/sources.list &&   \
#     echo 'deb https://mirrors.tuna.tsinghua.edu.cn/debian-security bullseye-security main contrib non-free' >> /etc/apt/sources.list

# # 2. Configure Rust registry

# RUN mkdir -p /usr/local/cargo &&   \
# printf '[source.crates-io]\nreplace-with = "tuna"\n\n[source.tuna]\nregistry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"\n' > /usr/local/cargo/config.toml

# 3. Install build dependencies

RUN apt-get update &&   \
apt-get install -y   \
ca-certificates   \
pkg-config   \
libssl-dev   \
build-essential &&   \
update-ca-certificates &&   \
rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 4. Copy dependency files first for better caching

COPY Cargo.toml Cargo.lock ./

# 5. Create dummy main.rs to build dependencies

RUN mkdir src && echo "fn main() {}" > src/main.rs

# 6. Build dependencies only

RUN cargo build --release && rm -rf src target/release/deps/proxy*

# 7. Copy actual source code

COPY src ./src

# 8. Build the actual application

RUN cargo build --release

# –– Runtime Stage ––

FROM debian:bookworm-slim

# 9. Configure TUNA mirror for runtime packages

# RUN echo 'deb https://mirrors.tuna.tsinghua.edu.cn/debian/ bookworm main contrib non-free non-free-firmware' > /etc/apt/sources.list &&   \
# echo 'deb https://mirrors.tuna.tsinghua.edu.cn/debian/ bookworm-updates main contrib non-free non-free-firmware' >> /etc/apt/sources.list &&   \
# echo 'deb https://mirrors.tuna.tsinghua.edu.cn/debian-security bookworm-security main contrib non-free non-free-firmware' >> /etc/apt/sources.list

# 10. Install runtime dependencies

RUN apt-get update &&   \
apt-get install -y   \
libssl3   \
ca-certificates   \
openssl \
libc6   \
curl \
iputils-ping \
&& rm -rf /var/lib/apt/lists/*

# 11. Create non-root user for security

RUN useradd -r -s /bin/false appuser

# 12. Copy binary and set permissions

COPY --from=builder /app/target/release/proxy /usr/local/bin/proxy
RUN chmod +x /usr/local/bin/proxy

# 13. Switch to non-root user

USER appuser

# 14. Set environment variables

ENV RUST_LOG=DEBUG
ENV RUST_BACKTRACE=1

# 15. Expose port if your app uses one (adjust as needed)

EXPOSE 8181

CMD ["/usr/local/bin/proxy"]
