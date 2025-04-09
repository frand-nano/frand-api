# 정보 요약
* **Nginx 설정:**
  - 역할: 리버스 프록시 (API 서버 및 Frontend 서비스)
  - 라우팅:
    - `/api`: API 서버로 프록시 패스 (`api` 서비스)
    - `/`: Yew 프론트엔드 정적 파일 서빙 (Nginx 내에서 처리)
  - 로깅: 표준 로그 형식 사용
  - TLS: Nginx 레벨에서 TLS 종료 처리
* **Docker 설정:**
  - 구성: API, Frontend + Nginx 별도 컨테이너로 구성
  - API 이미지: 멀티 스테이지 빌드 (`rust:latest` 빌드 -> `debian:bookworm-slim` 실행 환경)
  - Frontend 이미지: 멀티 스테이지 빌드 (Rust + Trunk 빌드 환경 -> Nginx 실행 환경)
    - Yew 빌드 결과물(HTML, JS, WASM, CSS 등)을 Nginx 이미지에 포함하여 정적 파일로 서빙
  - 관리: Docker Compose (`./deploy/docker-compose.yml`) 사용
  - 배포: 개발 환경에서 빌드된 이미지를 사용하여 `./deploy` 디렉토리만으로 배포 가능하도록 구성
  - 환경 변수: 각 서비스 및 Docker Compose 레벨에서 `.env` 파일 활용
  - 볼륨: TLS 인증서(`./deploy/secure/tls` 마운트), Nginx 설정 등 필요한 볼륨 마운트 설정
* **Yew Frontend 설정:**
  - 패키지: `./frontend` Cargo 패키지 추가
  - 프레임워크: Yew 사용
  - 스타일링: 직접 작성한 CSS 파일 (`style.css`) 사용
  - 라우팅: `yew-router` 크레이트 사용
  - API 통신: `.env` 파일로 API 엔드포인트 주소 관리
* **TLS 설정:**
  - 인증서 관리: 수동으로 발급받은 PEM 형식 인증서 사용 (`./deploy/secure/tls` 디렉토리에 위치)
  - 자동 갱신: 향후 추가 고려 (현재는 수동 관리)
* **기타:**
  - 데이터베이스 연동: 이번 단계에서는 포함하지 않음 (향후 추가)

# 구현 가이드
## 1. Frontend 패키지 설정 (`./frontend`)
  * `./frontend` 디렉토리 생성 및 Cargo 패키지 초기화.
  * `frontend/Cargo.toml` 수정:
    - Yew 및 라우터 등 필요한 의존성 추가 (상세 내용은 추가 파일 참조).
  * `frontend/src/main.rs` (또는 `lib.rs`) 기본 Yew 애플리케이션 구조 작성:
    - 애플리케이션 시작점, 기본 컴포넌트, 라우터 설정 포함.
  * `frontend/index.html` 생성:
    - Yew 앱 마운트 지점 및 CSS 파일 링크 포함 (상세 내용은 추가 파일 참조).
  * `frontend/style.css` 생성 및 기본 스타일 작성.
  * `frontend/.env.example` 파일 생성:
    - 프론트엔드 환경 변수 예시 정의 (상세 내용은 추가 파일 참조).
  * 루트 `Cargo.toml` 파일 수정:
    - 워크스페이스 멤버에 `frontend` 추가.

## 2. Dockerfile 설정
  * `api/Dockerfile` 생성 (상세 내용은 추가 파일 참조):
    - **빌드 스테이지:** Rust 환경에서 소스 코드를 빌드. 의존성 캐싱 활용.
    - **실행 스테이지:** 경량 Debian 이미지(`debian:bookworm-slim`)에 빌드 결과물 복사 및 실행 설정.
  * `frontend/Dockerfile` 생성 (상세 내용은 추가 파일 참조):
    - **빌드 스테이지:** Rust 환경에서 Trunk를 사용하여 Yew 앱 빌드.
    - **실행 스테이지:** Nginx 이미지에 빌드 결과물(정적 파일) 복사 및 실행 설정.

## 3. Nginx 설정 (`./deploy/nginx`)
  * `./deploy/nginx/conf.d/default.conf` 파일 생성 (상세 내용은 추가 파일 참조).
  * `server` 블록 설정:
    - HTTP(80) 및 HTTPS(443) 리스닝 설정.
    - 서버 이름 설정 (환경 변수 활용 권장).
    - **TLS 설정:** SSL 인증서 및 개인키 파일 경로 지정 (`/etc/nginx/certs/` 내부 파일 참조).
    - **루트 경로 및 정적 파일 서빙:** Frontend 빌드 결과물이 위치할 경로 지정 및 `index.html` 설정.
    - **API 프록시 설정:** `/api/` 경로 요청을 API 컨테이너로 전달하도록 `location` 블록 설정.
    - **Frontend 라우팅 처리:** 루트(`/`) 경로 요청 시 SPA 라우팅을 위해 `try_files` 설정.
    - **로깅 설정:** 접근 로그 및 오류 로그 파일 경로 지정.

## 4. Docker Compose 설정 (`./deploy`)
  * `./deploy/docker-compose.yml` 파일 생성 (상세 내용은 추가 파일 참조).
  * `services:` 정의:
    - **`api` 서비스:**
      - `api/Dockerfile`을 사용하여 빌드하도록 설정.
      - 컨테이너 이름, 환경 변수 파일, 네트워크, 재시작 정책 설정.
    - **`nginx` 서비스:**
      - `frontend/Dockerfile`을 사용하여 빌드하도록 설정 (Frontend 빌드 결과 포함).
      - 컨테이너 이름, 포트 매핑(80, 443), 볼륨 마운트(Nginx 설정, TLS 인증서 - `./secure/tls` 경로 사용), 서비스 의존성, 네트워크, 재시작 정책, 환경 변수 파일 설정.
  * `networks:` 정의:
    - 서비스 간 통신을 위한 브릿지 네트워크(`webnet`) 생성.

## 5. 환경 변수 설정
  * `api/.env`, `api/.env.test` 파일에 필요한 환경 변수 정의 (기존 `guide_01` 설정 유지).
  * `frontend/.env` 파일 생성 및 프론트엔드 관련 환경 변수 정의 (API 주소 등).
  * `deploy/.env` 파일 생성 및 Docker Compose, Nginx에서 사용할 환경 변수 정의 (도메인, TLS 인증서 파일명 등 - 경로 변수 제외).
  * 각 `.env.example` 파일에 해당 환경 변수 예시 제공 (상세 내용은 추가 파일 참조).

## 6. Yew Frontend 구현 (기본 라우팅 및 API 연동)
  * `frontend/src/main.rs` (또는 관련 모듈):
    - `yew-router`를 사용하여 기본적인 페이지 라우트 정의 (예: `/`, `/about`).
    - 각 라우트에 해당하는 컴포넌트 생성.
    - API 서버와 통신하는 로직 구현:
      - 환경 변수에서 API 기본 URL 읽어오기.
      - `web_sys::fetch` 또는 `reqwasm` 등을 사용하여 API 호출.
      - 상태 관리 로직 추가 (필요시).

## 7. 빌드 및 실행 확인
  * `./deploy/secure/tls` 디렉토리를 생성하고 유효한 `.pem` 형식의 TLS 인증서와 개인키 파일 준비.
  * 개발 환경에서 각 패키지 빌드 확인 (`cargo build -p api`, `trunk build frontend`).
  * Docker 이미지 빌드 확인 (`docker compose -f deploy/docker-compose.yml build`).
  * Docker Compose 실행 확인 (`docker compose -f deploy/docker-compose.yml up -d`).
  * 웹 브라우저에서 접속하여 프론트엔드 및 API 연동 확인 (HTTPS 접속 확인 포함).

# 추가 파일

1. `./frontend/Cargo.toml`
  ```toml
  [package]
  edition.workspace = true
  version.workspace = true
  authors.workspace = true
  license.workspace = true
  name = "frontend"

  [dependencies]
  yew = { version = "0.21", features = ["csr"] }
  yew-router = "0.18"
  wasm-bindgen = "0.2"
  web-sys = { version = "0.3", features = ["Window", "Document", "Element", "HtmlElement", "Node", "Location"] }
  ```

2. `./frontend/index.html`
  ```html
  <!DOCTYPE html>
  <html>
    <head>
      <meta charset="utf-8" />
      <title>Frand App</title>
      <link rel="stylesheet" href="/style.css">
      </head>
    <body>
      </body>
  </html>
  ```

3. `./frontend/style.css`
  ```css
  /* 기본 스타일 정의 */
  body {
    font-family: sans-serif;
  }
  ```

4. `./frontend/.env.example`
  ```dotenv
  # API 서버의 기본 URL (Nginx 리버스 프록시 경로)
  API_BASE_URL=/api
  ```

5. `./api/Dockerfile`
  ```dockerfile
  # --- 빌드 스테이지 ---
  FROM rust:1.77 AS builder

  WORKDIR /usr/src/app

  # 의존성 캐싱을 위해 Cargo.toml, Cargo.lock 먼저 복사
  COPY Cargo.toml ./
  COPY api/Cargo.toml api/Cargo.lock ./api/
  # 빈 lib.rs, main.rs 생성하여 의존성만 빌드
  RUN mkdir -p api/src && \
      echo "fn main() {}" > api/src/main.rs && \
      echo "pub fn create_rocket() -> rocket::Rocket<rocket::Build> { panic!() }" > api/src/lib.rs && \
      cargo build --release -p api --bin api_server && \
      rm -rf api/src

  # 실제 소스 코드 복사
  COPY api/src ./api/src

  # 애플리케이션 빌드
  RUN cargo build --release -p api --bin api_server

  # --- 실행 스테이지 ---
  FROM debian:bookworm-slim

  # 필요한 패키지 설치
  RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

  # 빌드된 바이너리 복사
  COPY --from=builder /usr/src/app/target/release/api_server /usr/local/bin/api_server

  # 환경 변수 설정 (필요시)
  # ENV ROCKET_ADDRESS=0.0.0.0
  # ENV ROCKET_PORT=8000

  # 포트 노출
  EXPOSE 8000

  # 실행 명령어
  CMD ["api_server"]
  ```

6. `./frontend/Dockerfile`
  ```dockerfile
  # --- 빌드 스테이지 ---
  FROM rust:1.77 AS builder

  # Trunk 설치
  RUN cargo install trunk --version 0.18.0

  WORKDIR /usr/src/app

  # 의존성 캐싱
  COPY frontend/Cargo.toml frontend/Cargo.lock ./frontend/
  RUN mkdir -p frontend/src && \
      echo "fn main() {}" > frontend/src/main.rs && \
      cargo build --release -p frontend && \
      rm -rf frontend/src

  # 소스 코드 및 정적 파일 복사
  COPY frontend/src ./frontend/src
  COPY frontend/index.html ./frontend/index.html
  COPY frontend/style.css ./frontend/style.css
  # .env 파일 복사 (빌드 시 사용)
  COPY frontend/.env ./frontend/.env

  # Trunk 빌드
  WORKDIR /usr/src/app/frontend
  RUN trunk build --release

  # --- 실행 스테이지 ---
  FROM nginx:stable-alpine

  # 빌드 결과물 복사
  COPY --from=builder /usr/src/app/frontend/dist /usr/share/nginx/html

  # Nginx 설정 파일 복사 (선택 사항, docker-compose에서 마운트할 수도 있음)
  # COPY deploy/nginx/conf.d/default.conf /etc/nginx/conf.d/default.conf

  # 포트 노출
  EXPOSE 80
  ```

7. `./deploy/nginx/conf.d/default.conf`
  ```nginx
  server {
      listen 80;
      listen 443 ssl http2;
      # server_name 값은 .env 또는 환경 변수로 주입하는 것이 좋음
      server_name ${NGINX_DOMAIN:-localhost};

      # TLS 설정
      ssl_certificate /etc/nginx/certs/${TLS_CERT_FILE:-localhost.pem};
      ssl_certificate_key /etc/nginx/certs/${TLS_KEY_FILE:-localhost-key.pem};
      # 기타 TLS 설정 (예: ssl_protocols, ssl_ciphers)

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
      container_name: frand_api
      env_file:
        - .env # api 서비스 자체 설정 + docker compose 변수
      networks:
        - webnet
      restart: unless-stopped

    nginx:
      # frontend 빌드 결과가 포함된 이미지를 직접 빌드
      build:
        context: ../frontend # frontend Dockerfile 사용 (빌드 + Nginx 스테이지 포함)
        dockerfile: Dockerfile
      container_name: frand_nginx
      ports:
        - "80:80"
        - "443:443"
      volumes:
        # Nginx 설정 파일 마운트
        - ./nginx/conf.d:/etc/nginx/conf.d:ro
        # TLS 인증서 마운트 (호스트 경로 변경)
        - ./secure/tls:/etc/nginx/certs:ro
      depends_on:
        - api
      networks:
        - webnet
      restart: unless-stopped
      env_file:
        - .env # nginx 설정 파일 내 변수 치환용

  networks:
    webnet:
      driver: bridge

  # 볼륨 정의는 이 구성에서는 필요하지 않습니다.
  # frontend 빌드 결과가 nginx 이미지에 포함되기 때문입니다.
  ```

9. `./deploy/.env.example`
  ```dotenv
  # Docker Compose 및 Nginx 설정용 환경 변수

  # Nginx 설정
  NGINX_DOMAIN=your_domain.com
  # TLS_CERT_PATH 변수는 docker-compose.yml에서 직접 경로를 지정하므로 제거됨
  TLS_CERT_FILE=your_cert.pem # 인증서 파일명 (./secure/tls 디렉토리 내 위치해야 함)
  TLS_KEY_FILE=your_key.pem   # 개인키 파일명 (./secure/tls 디렉토리 내 위치해야 함)

  # API 서비스 환경 변수 (필요시)
  # ROCKET_PORT=8000
  # ROCKET_ADDRESS=0.0.0.0
  ```