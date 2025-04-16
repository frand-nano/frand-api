# 빌드 스테이지
FROM rust:1.86-slim AS builder

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
COPY . .

# FRONTEND_API_ENDPOINT를 환경변수로 받음
ARG FRONTEND_API_ENDPOINT
ENV FRONTEND_API_ENDPOINT=$FRONTEND_API_ENDPOINT

# Yew 프론트엔드 빌드
WORKDIR /usr/src/app/yew
RUN trunk build --release

# 실행 스테이지 (Nginx)
FROM nginx:1.27.4-alpine-slim

# 정적 파일용 디렉토리 생성
RUN mkdir -p /usr/share/nginx/static

# 빌드된 정적 파일 복사
COPY --from=builder /usr/src/app/yew/dist /usr/share/nginx/html
# 정적 파일 복사
COPY --from=builder /usr/src/app/yew/static /usr/share/nginx/static

# Nginx 실행
CMD ["nginx", "-g", "daemon off;"]
