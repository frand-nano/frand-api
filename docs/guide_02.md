# 정보 요약
* **Nginx 설정:**
  - 역할: 리버스 프록시 (API 서버 및 Yew Frontend 서비스)
  - 라우팅:
    - `/api`: API 서버로 프록시 패스 (`api` 서비스)
    - `/`: Yew 프론트엔드 정적 파일 서빙 (Nginx 내에서 처리)
  - 로깅: 표준 로그 형식 사용
  - TLS: Nginx 레벨에서 TLS 종료 처리
* **Docker 설정:**
  - 구성: API, Yew Frontend + Nginx 별도 컨테이너로 구성
  - API 이미지: 멀티 스테이지 빌드 (`rust:latest` 빌드 -> `debian:bookworm-slim` 실행 환경)
  - Yew Frontend 이미지: 멀티 스테이지 빌드 (Rust + Trunk 빌드 환경 -> Nginx 실행 환경)
    - Yew 빌드 결과물(HTML, JS, WASM, CSS 등)을 Nginx 이미지에 포함하여 정적 파일로 서빙
  - 관리: Docker Compose (`./deploy/docker-compose.yml`) 사용
  - 배포: 개발 환경에서 빌드된 이미지를 사용하여 `./deploy` 디렉토리만으로 배포 가능하도록 구성
  - 환경 변수: 각 서비스 및 Docker Compose 레벨에서 `.env` 파일 활용
  - 볼륨: TLS 인증서(`./deploy/secrets/certs` 마운트), Nginx 설정 등 필요한 볼륨 마운트 설정
* **Yew Frontend 설정:**
  - 패키지: `./yew` Cargo 패키지 추가
  - 프레임워크: Yew 사용
  - 스타일링: 직접 작성한 CSS 파일 (`style.css`) 사용
  - 라우팅: `yew-router` 크레이트 사용
  - API 통신: `.env` 파일로 API 엔드포인트 주소 관리
* **TLS 설정:**
  - 인증서 관리: 수동으로 발급받은 PEM 형식 인증서 사용 (`./deploy/secrets/certs` 디렉토리에 위치)
  - 자동 갱신: 향후 추가 고려 (현재는 수동 관리)
* **기타:**
  - 데이터베이스 연동: 이번 단계에서는 포함하지 않음 (향후 추가)

# 구현 가이드
## 1. Yew Frontend 패키지 설정 (`./yew`)
  * `./yew` 디렉토리 생성 및 Cargo 패키지 초기화.
  * `yew/Cargo.toml` 수정:
    - Yew 및 라우터 등 필요한 의존성 추가 (상세 내용은 추가 파일 참조).
  * `yew/src/main.rs` (또는 `lib.rs`) 기본 Yew 애플리케이션 구조 작성:
    - 애플리케이션 시작점, 기본 컴포넌트, 라우터 설정 포함.
  * `yew/index.html` 생성:
    - Yew 앱 마운트 지점 및 CSS 파일 링크 포함 (상세 내용은 추가 파일 참조).
  * `yew/style.css` 생성 및 기본 스타일 작성.
  * `yew/.env.example` 파일 생성:
    - 프론트엔드 환경 변수 예시 정의 (상세 내용은 추가 파일 참조).
  * 루트 `Cargo.toml` 파일 수정:
    - 워크스페이스 멤버에 `yew` 추가.

## 2. Dockerfile 설정
  * `api/Dockerfile` 생성 (상세 내용은 추가 파일 참조):
    - **빌드 스테이지:** Rust 환경에서 소스 코드를 빌드. 의존성 캐싱 활용.
    - **실행 스테이지:** 경량 Debian 이미지(`debian:bookworm-slim`)에 빌드 결과물 복사 및 실행 설정.
  * `deploy/Dockerfile` 생성 (상세 내용은 추가 파일 참조):
    - **빌드 스테이지:** Rust 환경에서 Trunk를 사용하여 Yew 앱 빌드.
    - **실행 스테이지:** Nginx 이미지에 빌드 결과물(정적 파일) 복사 및 실행 설정.

## 3. Nginx 설정 (`./deploy/nginx`)
  * `./deploy/nginx/conf.d/default.conf` 파일 생성 (상세 내용은 추가 파일 참조).
  * `server` 블록 설정:
    - HTTP(80) 및 HTTPS(443) 리스닝 설정.
    - 서버 이름 설정 (환경 변수 활용 권장).
    - **TLS 설정:** SSL 인증서 및 개인키 파일 경로 지정 (`/etc/nginx/certs/` 내부 파일 참조).
    - **루트 경로 및 정적 파일 서빙:** Yew Frontend 빌드 결과물이 위치할 경로 지정 및 `index.html` 설정.
    - **API 프록시 설정:** `/api/` 경로 요청을 API 컨테이너로 전달하도록 `location` 블록 설정.
    - **Yew Frontend 라우팅 처리:** 루트(`/`) 경로 요청 시 SPA 라우팅을 위해 `try_files` 설정.
    - **로깅 설정:** 접근 로그 및 오류 로그 파일 경로 지정.

## 4. Docker Compose 설정 (`./deploy`)
  * `./deploy/docker-compose.yml` 파일 생성 (상세 내용은 추가 파일 참조).
  * `services:` 정의:
    - **`api` 서비스:**
      - `api/Dockerfile`을 사용하여 빌드하도록 설정.
      - 컨테이너 이름, 환경 변수 파일, 네트워크, 재시작 정책 설정.
    - **`nginx` 서비스:**
      - `deploy/Dockerfile`을 사용하여 빌드하도록 설정 (Yew Frontend 빌드 결과 포함).
      - 컨테이너 이름, 포트 매핑(80, 443), 볼륨 마운트(Nginx 설정, TLS 인증서 - `./secrets/certs` 경로 사용), 서비스 의존성, 네트워크, 재시작 정책, 환경 변수 파일 설정.
  * `networks:` 정의:
    - 서비스 간 통신을 위한 브릿지 네트워크(`frand-api-network`) 생성.

## 5. 환경 변수 설정
  * `api/.env`, `api/.env.test` 파일에 필요한 환경 변수 정의 (기존 `guide_01` 설정 유지).
  * `yew/.env` 파일 생성 및 프론트엔드 관련 환경 변수 정의 (API 주소 등).
  * `deploy/.env` 파일 생성 및 Docker Compose, Nginx에서 사용할 환경 변수 정의 (도메인, TLS 인증서 파일명 등 - 경로 변수 제외).
  * 각 `.env.example` 파일에 해당 환경 변수 예시 제공 (상세 내용은 추가 파일 참조).

## 6. Yew Frontend 구현 (기본 라우팅 및 API 연동)
  * `yew/src/main.rs` (또는 관련 모듈):
    - `yew-router`를 사용하여 기본적인 페이지 라우트 정의 (예: `/`, `/about`).
    - 각 라우트에 해당하는 컴포넌트 생성.
    - API 서버와 통신하는 로직 구현:
      - 환경 변수에서 API 기본 URL 읽어오기.
      - `web_sys::fetch` 또는 `reqwasm` 등을 사용하여 API 호출.
      - 상태 관리 로직 추가 (필요시).

## 7. 빌드 및 실행 확인
  * `./deploy/secrets/certs` 디렉토리를 생성하고 유효한 `.pem` 형식의 TLS 인증서와 개인키 파일 준비.
  * 개발 환경에서 각 패키지 빌드 확인 (`cargo build -p api`, `trunk build yew`).
  * Docker 이미지 빌드 확인 (`docker compose -f deploy/docker-compose.yml build`).
  * Docker Compose 실행 확인 (`docker compose -f deploy/docker-compose.yml up -d`).
  * 웹 브라우저에서 접속하여 프론트엔드 및 API 연동 확인 (HTTPS 접속 확인 포함).

# 추가 파일

1. `./yew/Cargo.toml`
  ```toml
  [package]
  version = "0.1.1"
  edition = "2021"
  license = "MIT"
  authors = [ "frand-nano <frand.nano@gmail.com>" ]
  name = "yew"

  [dependencies]
  yew = { version = "0.21", features = ["csr"] }
  yew-router = "0.18"
  wasm-bindgen = "0.2"
  web-sys = { version = "0.3", features = ["Window", "Document", "Element", "HtmlElement", "Node", "Location"] }
  ```

2. `./yew/index.html`
  ```html
  <!DOCTYPE html>
  <html>
    <head>
      <meta charset="utf-8" />
      <title>Frand App</title>
      <link rel="stylesheet" href="/style.css">
    </head>
    <body>
      <div id="app"></div>
    </body>
  </html>
  ```

3. `./yew/style.css`
  ```css
  /* 기본 스타일 정의 */
  body {
    font-family: sans-serif;
  }
  ```

4. `./yew/.env.example`
  ```dotenv
  # API 서버의 기본 URL (Nginx 리버스 프록시 경로)
  API_BASE_URL=/api
  ```

5. `./api/Dockerfile`
  ```dockerfile
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
  ```

6. `./deploy/Dockerfile`
  ```dockerfile
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
  ```

7. `./deploy/nginx/conf.d/default.conf`
  ```nginx
  server {
      listen 80;
      listen 443 ssl http2;
      
      # server_name 값은 환경변수로 주입됨
      server_name ${NGINX_DOMAIN};

      # TLS 설정
      ssl_certificate /etc/nginx/certs/${TLS_CERT_FILE};
      ssl_certificate_key /etc/nginx/certs/${TLS_KEY_FILE};

      # 로깅
      access_log /var/log/nginx/access.log;
      error_log /var/log/nginx/error.log;

      # 정적 파일 루트
      root /usr/share/nginx/html;
      index index.html;

      # API 프록시
      location /api/ {
          # Docker Compose 서비스 이름 사용
          proxy_pass http://api:8000/;
          proxy_set_header Host $host;
          proxy_set_header X-Real-IP $remote_addr;
          proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
          proxy_set_header X-Forwarded-Proto $scheme;
      }

      # 프론트엔드 라우팅 처리
      location / {
          try_files $uri $uri/ /index.html;
      }
  }
  ```

8. `./deploy/docker-compose.yml`
  ```yaml
  services:
    api:
      build:
        context: ../api
        dockerfile: Dockerfile
      image: frand-api:latest
      env_file:
        - .env # api 서비스 자체 설정 + docker compose 변수
      networks:
        - frand-api-network
      restart: unless-stopped

    nginx:
      build:
        context: ..
        dockerfile: deploy/nginx/Dockerfile
      image: frand-api-nginx:latest
      ports:
        - "80:80"
        - "443:443"
      volumes:
        # TLS 인증서 마운트
        - ./secrets/certs:/etc/nginx/certs:ro
      depends_on:
        - api
      networks:
        - frand-api-network
      restart: unless-stopped
      env_file:
        - .env # nginx 설정 파일 내 변수 치환용
      environment:
        # 기본값 설정 (필요시 .env로 오버라이드)
        - NGINX_DOMAIN=localhost
        - TLS_CERT_FILE=cert.pem
        - TLS_KEY_FILE=privkey.pem

  networks:
    frand-api-network:
      driver: bridge
  ```

9. `./deploy/nginx/Dockerfile`
  ```dockerfile
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
  ```

10. `./deploy/nginx/docker-entrypoint.sh`
  ```bash
  #!/bin/sh
  set -e

  # Nginx 설정 파일의 환경변수를 치환
  envsubst '${NGINX_DOMAIN} ${TLS_CERT_FILE} ${TLS_KEY_FILE}' < /etc/nginx/conf.d/default.template > /etc/nginx/conf.d/default.conf

  # 기본 nginx 엔트리포인트 실행
  exec "$@"
  ```

11. `./deploy/.env.example`
  ```dotenv
  # Docker Compose 및 Nginx 설정용 환경 변수

  # Nginx 설정
  NGINX_DOMAIN=localhost
  TLS_CERT_FILE=cert.pem
  TLS_KEY_FILE=privkey.pem

  # API 서비스 환경 변수
  ROCKET_PORT=8000
  ROCKET_ADDRESS=0.0.0.0
  ```

12. `./deploy/generate-certs.sh`
  ```bash
  #!/bin/bash

  # 자체 서명 인증서 생성 스크립트
  # 이 스크립트는 Nginx에서 사용할 TLS 인증서와 개인 키를 생성합니다.

  # 색상 코드 정의
  RED='\033[0;31m'
  GREEN='\033[0;32m'
  YELLOW='\033[0;33m'
  NC='\033[0m' # No Color

  # 환경 변수 로드
  if [ -f .env ]; then
      source .env
      DOMAIN=$NGINX_DOMAIN
  else
      DOMAIN="localhost"
      echo -e "${YELLOW}경고: .env 파일을 찾을 수 없습니다. 도메인을 'localhost'로 설정합니다.${NC}"
  fi

  # 인증서 디렉토리 생성
  CERT_DIR="./secrets/certs"
  if [ ! -d "$CERT_DIR" ]; then
      mkdir -p "$CERT_DIR"
      echo "인증서 디렉토리 생성: $CERT_DIR"
  fi

  # OpenSSL 설치 확인
  if ! command -v openssl &> /dev/null; then
      echo -e "${RED}오류: OpenSSL이 설치되어 있지 않습니다.${NC}"
      echo "Ubuntu/Debian: sudo apt-get install openssl"
      echo "CentOS/RHEL: sudo yum install openssl"
      echo "macOS: brew install openssl"
      exit 1
  fi

  echo -e "${GREEN}자체 서명 인증서 생성 시작...${NC}"

  # 개인 키 및 인증서 경로 설정
  CERT_PATH="$CERT_DIR/cert.pem"
  KEY_PATH="$CERT_DIR/privkey.pem"

  # 개인 키 및 자체 서명 인증서 생성
  openssl req -x509 -newkey rsa:4096 -nodes \
      -keyout "$KEY_PATH" \
      -out "$CERT_PATH" \
      -days 365 \
      -subj "/C=KR/ST=Seoul/L=Seoul/O=Frand API/OU=Development/CN=$DOMAIN" \
      -addext "subjectAltName = DNS:$DOMAIN,DNS:www.$DOMAIN,IP:127.0.0.1"

  # 생성 확인
  if [ -f "$CERT_PATH" ] && [ -f "$KEY_PATH" ]; then
      echo -e "${GREEN}인증서 생성 완료!${NC}"
      echo "인증서 위치: $CERT_PATH"
      echo "개인 키 위치: $KEY_PATH"
      
      # 인증서 정보 출력
      echo -e "\n${YELLOW}인증서 정보:${NC}"
      openssl x509 -in "$CERT_PATH" -noout -text | grep -E 'Subject:|Issuer:|Not Before:|Not After :|DNS:'
      
      # 필요한 경우 권한 설정
      chmod 644 "$CERT_PATH"
      chmod 600 "$KEY_PATH"
  else
      echo -e "${RED}인증서 생성 실패!${NC}"
      exit 1
  fi
  ```

13. `./yew/trunk.toml`
  ```toml
  [build]
  # Build in release mode
  release = false

  [watch]
  # Watch these directories for changes
  watch = ["src", "style.css", "index.html"]

  [serve]
  # The address to serve on
  address = "127.0.0.1"
  # The port to serve on
  port = 8080
  # Open a browser tab once the server started
  open = true
  ```