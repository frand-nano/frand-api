# 빌드 스테이지
FROM rust:1.86-slim AS builder

WORKDIR /usr/src/app

# 종속성 파일만 먼저 복사하여 캐싱 활용
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/
COPY yew/Cargo.toml yew/

# 빌드를 위한 임시 소스 파일 생성 (캐싱 활용)
RUN mkdir -p api/src yew/src && \
    echo "fn main() {}" > api/src/main.rs && \
    echo "pub fn dummy() {}" > yew/src/lib.rs && \
    cargo build --release --package api && \
    rm -rf api/src

# 실제 소스 코드 복사
COPY api api/

# 앱 빌드 (캐싱된 종속성 활용)
RUN touch api/src/main.rs && \
    cargo build --release --package api

# 실행 스테이지
FROM debian:bookworm-slim

# 필요한 패키지 설치
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# 빌드된 바이너리만 복사
COPY --from=builder /usr/src/app/target/release/api .

# 실행 명령
CMD ["api"]
