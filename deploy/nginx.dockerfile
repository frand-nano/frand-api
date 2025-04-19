# 빌드 스테이지
FROM rust:1.86-slim AS builder

# ROCKET_API_ENDPOINT를 환경변수로 받음
ARG ROCKET_API_ENDPOINT
ENV ROCKET_API_ENDPOINT=$ROCKET_API_ENDPOINT

# Trunk 및 필요한 도구 설치
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl ca-certificates pkg-config libssl-dev && \
    curl -sL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    cargo install trunk wasm-bindgen-cli && \
    rustup target add wasm32-unknown-unknown && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# 종속성 파일만 먼저 복사하여 캐싱 활용
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml ./api/
COPY yew/Cargo.toml ./yew/
COPY common/Cargo.toml ./common/

# 빌드를 위한 임시 소스 파일 생성 (캐싱 활용)
RUN mkdir -p api/src yew/src common/src && \
    echo "fn lib() {}" > api/src/lib.rs && \
    echo "fn main() {}" > yew/src/main.rs && \
    echo "fn lib() {}" > common/src/lib.rs && \
    cargo build --release --package frand-api-yew

# 실제 소스 파일 복사
COPY Trunk.toml ./
COPY yew ./yew
COPY common ./common

# Yew 프론트엔드 빌드
RUN trunk build --release

# 실행 스테이지 (Nginx)
FROM nginx:1.27.4-alpine-slim

# 정적 파일용 디렉토리 생성
RUN mkdir -p /usr/share/nginx/static

# 빌드된 정적 파일 복사
COPY --from=builder /usr/src/app/yew/dist /usr/share/nginx/yew/dist

# Nginx 실행
CMD ["nginx", "-g", "daemon off;"]