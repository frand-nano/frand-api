# --- 빌드 스테이지 ---
FROM rust:1.86-slim AS builder

WORKDIR /usr/src/app

# 의존성 캐싱을 위해 Cargo.toml, Cargo.lock 먼저 복사
COPY Cargo.toml Cargo.lock ./

# 빈 lib.rs, main.rs 생성하여 의존성만 빌드
RUN mkdir -p src && \
    echo "fn main() { unreachable!() }" > src/main.rs && \
    echo "" > src/lib.rs && \
    cargo build --release --bin api_server && \
    rm -rf src


# 실제 소스 코드 복사
COPY src ./src

# main.rs, lib.rs를 touch하여 캐시 리셋
RUN touch src/main.rs src/lib.rs

# 애플리케이션 빌드
RUN cargo build --release --bin api_server

# --- 실행 스테이지 ---
FROM debian:bookworm-slim

# 필요한 패키지 설치
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# 빌드된 바이너리 복사
COPY --from=builder /usr/src/app/target/release/api_server /usr/local/bin/api_server

# 실행 명령어
CMD ["api_server"]
