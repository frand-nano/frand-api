# --- 프론트엔드 빌드 스테이지 ---
FROM rust:1.86-slim AS yew-builder

# 필요한 시스템 패키지 설치
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Trunk 설치
RUN cargo install trunk

# WebAssembly 타겟 추가
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app

# 의존성 캐싱
COPY yew/Cargo.toml yew/Cargo.lock ./
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 소스 코드 및 정적 파일 복사
COPY yew/src ./src
COPY yew/index.html ./index.html
COPY yew/style.css ./style.css

# .env 파일 복사 (빌드 시 사용)
COPY yew/.env ./.env

RUN trunk build --release

# --- Nginx 스테이지 ---
FROM nginx:1.27.4-alpine-slim

# envsubst 명령을 사용하기 위한 패키지 설치
RUN apk add --no-cache gettext

# 기본 Nginx 설정 제거
RUN rm /etc/nginx/conf.d/default.conf

# 엔트리포인트 스크립트 복사 및 실행 권한 부여
COPY deploy/nginx/docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# 설정 파일 템플릿 복사
COPY deploy/nginx/conf.d/default.conf /etc/nginx/conf.d/default.template

# 프론트엔드 빌드 결과물 복사
COPY --from=yew-builder /usr/src/app/dist /usr/share/nginx/html

ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
CMD ["nginx", "-g", "daemon off;"]
