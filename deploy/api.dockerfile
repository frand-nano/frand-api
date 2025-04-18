# 빌드 스테이지
FROM rust:1.86-slim AS builder

WORKDIR /usr/src/app

# 종속성 파일만 먼저 복사하여 캐싱 활용
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/

# 빌드를 위한 임시 소스 파일 생성 (캐싱 활용)
RUN mkdir -p api/src && \
    echo "fn main() {}" > api/src/main.rs && \
    cargo build --release --package api && \
    rm -f api/src/main.rs

# 실제 소스 파일 복사
COPY api ./api

# 빌드
RUN touch api/src/main.rs && \
    cargo build --release --package api

# 실행 스테이지
FROM debian:bookworm-slim

# 필요한 패키지 설치
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin/api

COPY --from=builder /usr/src/app/target/release/api ./frand_api
COPY ./.env ./.env

# 실행 명령
CMD ["./frand_api"]