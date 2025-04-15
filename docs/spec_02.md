# 정보 요약
* **프로그램 유형:** Rust 기반 REST API 서버 (`frand-api` 워크스페이스 구조) + Yew 기반 웹 애플리케이션 + MongoDB 데이터베이스
* **주요 기술:**
  * Backend: Rust (`1.86` 기반), Rocket (`0.5.1`), Tokio (`1.37.0`), Serde, dotenvy, log/simple_logger, MongoDB Rust Driver (`mongodb`)
  * Frontend (Yew): Rust (`1.86` 기반), Yew (`0.21`), Trunk, CSS (`style.css`), yew-router (`0.18`)
  * 데이터베이스: MongoDB (`6.0` 권장)
  * 인프라: Docker, Docker Compose, Nginx (`nginx:1.27.4-alpine-slim` 기반)
* **주요 기능:**
  * API: 기본 설정 및 health check (`/api/v1/health`), 루트 경로 응답 (`/`), MongoDB 연동 (향후 구체화)
  * Yew Frontend: Yew 기반 웹 애플리케이션 (`/` 경로에 `HomePage` 컴포넌트 렌더링)
  * DB: 데이터 영속성 관리 (명명된 볼륨 `mongo-data` 사용)
* **폴더 구조:**
  * 워크스페이스 루트 (`frand-api/`)
    * `api/`: API 서버 패키지
    * `yew/`: Yew Frontend 패키지 (`Trunk.toml`, `static/` 포함)
    * `deploy/`: 배포 관련 파일 (Dockerfiles, `docker-compose.yml`, Nginx 설정, TLS 인증서)
    * `.env`, `.env.test`: 환경 변수 파일
* **데이터베이스:**
  * MongoDB 사용 (`mongo` 서비스 이름으로 Docker 네트워크 내에서 접근).
  * API 서비스는 `DATABASE_HOST=mongo` 환경 변수 사용.
  * 인증 정보 (`DATABASE_USER`, `DATABASE_PASS`)는 `.env` 파일에서 관리.
  * 데이터 영속성을 위해 Docker 명명된 볼륨 (`mongo-data`) 사용.
* **설정 관리:**
  * 프로젝트 루트 `.env`, `.env.test` 파일 (API, Docker Compose, DB 공용)
  * `api/src/config.rs` 모듈 (API 설정 로드)
  * Yew Frontend 설정 (API 엔드포인트 등):
    * `.env` 파일의 `FRONTEND_API_ENDPOINT` 값을 Docker 빌드 인자로 전달 (`--build-arg`).
    * `yew.Dockerfile` 빌드 단계에서 환경 변수로 설정 (`ENV`).
    * `yew/Trunk.toml`의 `[env]` 섹션을 통해 Yew 코드에서 `std::env!("FRONTEND_API_ENDPOINT")`로 접근.
* **API 버전 관리:** URL 경로 사용 (`/api/v1/`)
* **데이터 형식:** JSON (API 요청/응답)
* **API 응답 구조:** 표준 구조 사용 (`ApiResponse<T>`, `ApiError`)
* **오류 처리:** Rocket catcher 사용 (`ApiError` 형식으로 일관된 오류 응답)
* **로깅:**
  * API: `simple_logger` 사용 (`LOG_LEVEL` 환경 변수로 제어)
  * Nginx: 표준 로그 형식 사용 (추가 설정 없음)
* **테스트:**
  * API: 통합 테스트 (`api/tests/`), 단위 테스트 (필요시)
  * Yew Frontend: (테스트 전략 미정)
* **배포:**
  * Docker Compose (`./deploy/docker-compose.yml`) 사용 (`frand-api-network` 네트워크 사용).
  * API, Yew Frontend (Nginx), MongoDB 별도 컨테이너.
  * 멀티 스테이지 Docker 빌드 (API, Yew Frontend).
  * 개발 환경 빌드 이미지 사용, `./deploy` 디렉토리만으로 배포 가능.
  * Nginx 설정: `./deploy/nginx/nginx.conf` 사용.
    * 역할: 리버스 프록시 (`/api` -> `api` 서비스), Yew 정적 파일 서빙 (`/` -> Yew 빌드 결과물), TLS 종료.
    * 프록시 헤더: 표준 헤더 설정 (`Host`, `X-Real-IP`, `X-Forwarded-For`, `X-Forwarded-Proto`).
    * 캐싱: 최소한의 캐싱 또는 캐싱 없음.
  * 볼륨 마운트:
    * Nginx 설정: `./deploy/nginx/nginx.conf` -> `/etc/nginx/conf.d/default.conf` (읽기 전용).
    * TLS 인증서: `./deploy/secure/tls` -> `/etc/nginx/certs` (읽기 전용).
    * MongoDB 데이터: `mongo-data` (명명된 볼륨) -> `/data/db`.
  * TLS:
    * Nginx 레벨에서 TLS 종료 처리.
    * 수동 발급된 PEM 형식 인증서 사용 (`cert.pem`, `privkey.pem` 파일).
    * 인증서 경로 설정: Docker 볼륨 마운트 (`/etc/nginx/certs`) 및 Nginx 설정 파일에서 직접 경로 참조.
    * (개발/테스트 시 자체 서명 인증서 생성 스크립트 활용 가능).

# 구현 명세

## 1. 프로젝트 구조

```
frand-api/
├── .env                # 공용 환경 변수 설정 파일 (API, Docker, DB, Yew 빌드 시 사용)
├── .env.test           # 테스트 환경 변수 설정 파일
├── .gitignore
├── Cargo.toml          # 워크스페이스 설정 파일 (members = ["api", "yew"])
├── api/                # API 서버 패키지
│   ├── Cargo.toml      # mongodb 의존성 추가
│   ├── src/            # config.rs 에서 DB 연결 정보 처리
│   └── tests/
├── yew/                # Yew Frontend 패키지
│   ├── Cargo.toml
│   ├── Trunk.toml      # [env] 섹션에 FRONTEND_API_ENDPOINT 정의
│   ├── static/         # 정적 에셋 (예: style.css) - Trunk 가 빌드 시 포함
│   │   └── style.css
│   └── src/            # std::env!("FRONTEND_API_ENDPOINT") 사용
├── deploy/             # 배포 관련 파일
│   ├── docker-compose.yml # api, yew_frontend, mongo 서비스 정의
│   ├── api.Dockerfile     # API 서비스 Dockerfile (debian:bookworm-slim 기반)
│   ├── yew.Dockerfile     # Yew Frontend 서비스 Dockerfile (nginx:1.27.4-alpine-slim 기반)
│   ├── nginx/
│   │   └── nginx.conf     # Nginx 설정 파일 (/etc/nginx/certs 경로 사용)
│   └── secure/
│       └── tls/          # TLS 인증서 저장 위치 (호스트)
│           ├── cert.pem
│           └── privkey.pem
└── README.md
```
* **참고:** Yew 빌드 결과물은 `./yew/dist`에 생성되며, `yew.Dockerfile`에서 Nginx 이미지 내부(`/usr/share/nginx/html`)로 복사됩니다. `yew/static/style.css`는 Trunk가 빌드 시 자동으로 포함시킵니다.

## 2. 주요 파일 역할 및 의존성

* **`api/Cargo.toml`**: `[dependencies]` 섹션에 `mongodb` 크레이트 추가.
* **`api/src/config.rs`**: MongoDB 연결 정보(`DATABASE_USER`, `DATABASE_PASS`, `DATABASE_HOST`, `DATABASE_PORT`)를 환경 변수에서 읽어 `Config` 구조체에 포함하고, 연결 문자열 생성 로직 추가 (향후 DB 연동 구현 시).
* **`frand-api/.env`**: `DATABASE_USER`, `DATABASE_PASS` 변수 추가. `DATABASE_HOST`, `DATABASE_PORT`는 필요시 기본값 설정 가능 (docker-compose에서 오버라이드). `FRONTEND_API_ENDPOINT` 변수 포함.
* **`yew/Trunk.toml`**: `[env]` 섹션에 `FRONTEND_API_ENDPOINT = {}` 명시하여 빌드 시 환경 변수 주입 활성화.
* **`yew/src/lib.rs`**: API 요청 시 `std::env!("FRONTEND_API_ENDPOINT")`를 사용하여 기본 URL 구성.
* **`deploy/docker-compose.yml`**: `mongo` 서비스 추가. `api` 서비스에 `DATABASE_HOST=mongo` 환경 변수 및 `mongo` 의존성 추가. `yew_frontend` 서비스(Nginx)의 TLS 볼륨 마운트 경로 수정 (`/etc/nginx/certs`). `mongo` 서비스에 인증 환경 변수 및 데이터 볼륨 설정 추가. `yew_frontend` 서비스의 Dockerfile 경로 수정.
* **`deploy/api.Dockerfile`**: 실행 스테이지 베이스 이미지 `debian:bookworm-slim` 사용. `ca-certificates` 설치 확인. 바이너리 이름 `api_server` 확인.
* **`deploy/yew.Dockerfile`**: 빌드 스테이지 베이스 이미지 `rust:1.86-slim` 사용. `ARG FRONTEND_API_ENDPOINT`, `ENV FRONTEND_API_ENDPOINT=$FRONTEND_API_ENDPOINT` 추가. 실행 스테이지 베이스 이미지 `nginx:1.27.4-alpine-slim` 사용. Yew 빌드 결과물을 Nginx 이미지로 복사.
* **`deploy/nginx/nginx.conf`**: TLS 인증서 경로를 `/etc/nginx/certs/cert.pem`, `/etc/nginx/certs/privkey.pem` 으로 수정.

## 3. 설정 파일 (`Cargo.toml`, `.env`, `docker-compose.yml`, `nginx.conf`, `Trunk.toml`)

* **`frand-api/Cargo.toml`**: 워크스페이스 레벨 설정.
* **`api/Cargo.toml`**:
    ```toml
    # filepath: /api/Cargo.toml
    [package]
    name = "api"
    version.workspace = true
    edition.workspace = true
    license.workspace = true
    authors.workspace = true
    publish = false

    [dependencies]
    rocket = { workspace = true, features = ["json"] }
    tokio = { workspace = true }
    serde = { workspace = true }
    serde_json = { workspace = true }
    dotenvy = { workspace = true }
    log = { workspace = true }
    simple_logger = { workspace = true }
    mongodb = "2.8.2" # MongoDB 드라이버 추가 (버전 확인 필요)

    ```
* **`yew/Cargo.toml`**: Yew 관련 의존성 정의.
* **`.env` (예시)**:
    ```dotenv
    # filepath: /.env

    # API Service
    ROCKET_ADDRESS=0.0.0.0
    API_INTERNAL_PORT=8000
    LOG_LEVEL=info
    # DATABASE_HOST=localhost # 기본값 (Docker Compose 에서 오버라이드)
    # DATABASE_PORT=27017   # 기본값
    DATABASE_USER=root
    DATABASE_PASS=example_password

    # Yew Frontend Service (Nginx)
    NGINX_EXTERNAL_PORT=443
    NGINX_EXTERNAL_HTTP_PORT=80

    # Yew Application Build Argument
    FRONTEND_API_ENDPOINT=/api/v1
    ```
* **`deploy/docker-compose.yml`**:
    ```yaml
    # filepath: /deploy/docker-compose.yml
    version: '3.8'

    services:
      api:
        build:
          context: .. # 워크스페이스 루트 기준
          dockerfile: deploy/api.Dockerfile
        image: frand-api:latest # 이미지 이름 지정
        container_name: frand_api_service
        restart: unless-stopped
        env_file:
          - ../.env
        environment:
          # Docker 내부 네트워크에서 mongo 서비스 이름으로 접근하도록 호스트 오버라이드
          DATABASE_HOST: mongo
        networks:
          - frand-api-network
        volumes:
          - /etc/localtime:/etc/localtime:ro
        depends_on:
          - mongo # API 시작 전 MongoDB 준비 완료 대기

      yew_frontend: # 서비스 이름 변경 (frontend -> yew_frontend)
        build:
          context: .. # 워크스페이스 루트 기준
          dockerfile: deploy/yew.Dockerfile # Dockerfile 경로 변경
          args:
            - FRONTEND_API_ENDPOINT=${FRONTEND_API_ENDPOINT}
        image: frand-api-yew-nginx:latest # 이미지 이름 변경
        container_name: frand_yew_nginx # 컨테이너 이름 변경
        restart: unless-stopped
        ports:
          - "${NGINX_EXTERNAL_PORT}:443"
          - "${NGINX_EXTERNAL_HTTP_PORT}:80"
        volumes:
          - ./nginx/nginx.conf:/etc/nginx/conf.d/default.conf:ro
          - ./secure/tls:/etc/nginx/certs:ro # TLS 인증서 마운트 (경로 수정)
          - /etc/localtime:/etc/localtime:ro
        depends_on:
          - api
        networks:
          - frand-api-network

      mongo:
        image: mongo:6.0 # 특정 버전 사용 권장
        container_name: frand_mongo_db
        restart: unless-stopped
        environment:
          MONGO_INITDB_ROOT_USERNAME: ${DATABASE_USER}
          MONGO_INITDB_ROOT_PASSWORD: ${DATABASE_PASS}
        volumes:
          - mongo-data:/data/db # 명명된 볼륨 사용
        # ports: # 외부 직접 접근 불필요 시 주석 처리
        #   - "27017:27017"
        networks:
          - frand-api-network

    networks:
      frand-api-network:
        driver: bridge

    volumes:
      mongo-data: # MongoDB 데이터 영속화를 위한 볼륨
    ```
* **`deploy/nginx/nginx.conf`**:
    ```nginx
    # filepath: /deploy/nginx/nginx.conf

    server {
        listen 80;
        server_name _; # 실제 운영 시 도메인으로 변경

        # HTTP -> HTTPS 리다이렉션
        location / {
            return 301 https://$host$request_uri;
        }
    }

    server {
        listen 443 ssl http2;
        server_name _; # 실제 운영 시 도메인으로 변경

        # TLS 설정 (경로 수정)
        ssl_certificate /etc/nginx/certs/cert.pem;
        ssl_certificate_key /etc/nginx/certs/privkey.pem;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384;
        ssl_prefer_server_ciphers off;

        # API 프록시 설정
        location /api/ {
            proxy_pass http://api:${API_INTERNAL_PORT}/api/;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
        }

        # Yew Frontend 정적 파일 서빙 설정
        location / {
            root /usr/share/nginx/html;
            try_files $uri $uri/ /index.html;
            expires -1;
            add_header Cache-Control "no-store, no-cache, must-revalidate, proxy-revalidate, max-age=0";
        }

        # 로깅
        access_log /var/log/nginx/access.log;
        error_log /var/log/nginx/error.log;
    }
    ```
* **`yew/Trunk.toml`**:
    ```toml
    # filepath: /yew/Trunk.toml

    [build]
    dist = "dist"
    public_url = "/"

    [env]
    FRONTEND_API_ENDPOINT = {}

    [watch]
    ignore = ["../target", "../deploy"]

    [serve]
    address = "127.0.0.1"
    port = 8080
    open = true
    [proxy]
    backend = "http://localhost:8000/api/"
    ```

## 4. 핵심 구조체 및 함수 시그니처

(API, Yew Frontend 및 MongoDB 연동 부분은 컴포넌트 및 데이터 모델 구현 후 필요시 추가)

## 5. API 엔드포인트

(향후 MongoDB 연동 기능 추가 시 관련 엔드포인트 정의 필요)

## 6. 오류 처리

(API 오류 처리는 `api/src/error.rs` 정의 및 Rocket catcher 사용)

## 7. 구현 가이드라인

* **Docker 빌드**: 각 Dockerfile은 멀티 스테이지 빌드를 활용하여 최종 이미지 크기를 최적화. `yew.Dockerfile` 빌드 시 `--build-arg`를 사용하여 `.env`의 값을 전달하고, Trunk가 이를 사용하도록 설정. 명시된 베이스 이미지 버전 사용 권장 (`rust:1.86-slim`, `nginx:1.27.4-alpine-slim`, `debian:bookworm-slim`).
* **Docker Compose**: `depends_on`을 사용하여 서비스 시작 순서 제어 (`yew_frontend` -> `api` -> `mongo`). `restart: unless-stopped` 정책 사용. 볼륨 마운트는 필요한 설정, 보안 파일, 데이터 영속성(`mongo-data`)에 대해서만 사용 (읽기 전용 권장). 이미지 버전 고정 권장 (`mongo:6.0`).
* **Nginx**: TLS 종료, API 리버스 프록시, Yew Frontend 정적 파일 서빙 역할 수행. `try_files`를 사용하여 Yew 라우터 지원. TLS 설정은 마운트된 인증서 경로(`/etc/nginx/certs`) 직접 참조.
* **Yew 빌드**: `Trunk`를 사용하여 Yew 애플리케이션 빌드. `Trunk.toml`에서 빌드 경로, public URL, 환경 변수 설정 관리. `yew/static` 폴더의 에셋은 Trunk가 자동으로 처리.
* **환경 변수 관리**: `.env` 파일을 중심으로 환경 변수 관리. API 서버는 `dotenvy`로 직접 로드, Docker Compose는 `env_file` 및 `environment`, `args`로 사용, Yew 앱은 Trunk의 `[env]` 기능을 통해 빌드 시점에 주입받음.
* **MongoDB 연동**: API 서버는 `mongodb` 드라이버를 사용하여 `mongo` 서비스에 연결. 연결 정보는 `Config` 구조체를 통해 관리.
* **정적 파일**: Yew 빌드 결과물(`dist` 폴더 내용)은 `yew.Dockerfile`에서 Nginx 이미지로 복사되어 서빙됨.
* **Docker 이미지 환경**: API 및 Yew 빌드/실행에 필요한 시스템 패키지(`ca-certificates` 등)는 각 Dockerfile에 명시. 추가 필요시 구현 단계에서 식별하여 반영.
* **자체 서명 인증서 (개발용)**: 개발 또는 테스트 환경에서는 OpenSSL 등을 이용한 자체 서명 인증서 생성 스크립트(예: 이전 제공된 `generate_certs.sh` 수정본)를 활용하여 `./deploy/secure/tls` 위치에 인증서를 생성하는 것을 고려할 수 있음.

