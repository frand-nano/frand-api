# 정보 요약
* **프로그램 유형:** Rust 기반 REST API 서버 (`frand-api` 워크스페이스 구조) + Yew 기반 웹 애플리케이션 + MongoDB 데이터베이스
* **주요 기술:**
  * Backend: Rust (`1.86` 기반), Rocket (`0.5.1`), Tokio (`1.37.0`), Serde, dotenvy, log/simple_logger, MongoDB Rust Driver (`mongodb`)
  * Frontend (Yew): Rust (`1.86` 기반), Yew (`0.21`), Trunk, CSS (`style.css`), yew-router (`0.18`)
  * 데이터베이스: MongoDB (`6.0` 권장)
  * 인프라: Docker, Docker Compose, Nginx (`nginx:1.27.4-alpine-slim` 기반)
* **주요 기능:**
  * API: 기본 설정 및 health check (`/api/v1/health`), MongoDB 연동 (향후 구체화)
  * Yew Frontend: Yew 기반 웹 애플리케이션 (`/` 경로에 `HomePage` 컴포넌트 렌더링)
  * DB: 데이터 영속성 관리 (명명된 볼륨 `mongo-data` 사용)
* **폴더 구조:**
  * 워크스페이스 루트 (`frand-api/`)
    * `api/`: API 서버 패키지
    * `yew/`: Yew Frontend 패키지 (`Trunk.toml`, `static/`, `index.html`, `src/` 포함)
    * `deploy/`: 배포 관련 파일 (Dockerfiles, `docker-compose.yml`, Nginx 설정, TLS 인증서, `.env.example`, `gen_certs.sh`)
    * `.env`, `.env.test`: 환경 변수 파일
    * `.env.example`: 환경 변수 예시 파일
    * `.gitignore`: Git 무시 파일 목록
    * `.dockerignore`: Docker 빌드 시 무시 파일 목록
* **데이터베이스:**
  * MongoDB 사용 (`mongo` 서비스 이름으로 Docker 네트워크 내에서 접근).
  * API 서비스는 `DATABASE_HOST=mongo` 환경 변수 사용.
  * 인증 정보 (`DATABASE_USER`, `DATABASE_PASS`)는 `.env` 파일에서 관리.
  * 데이터 영속성을 위해 Docker 명명된 볼륨 (`mongo-data`) 사용.
* **설정 관리:**
  * 프로젝트 루트 `.env`, `.env.test`, `.env.example` 파일 (API, Docker Compose, DB 공용)
  * `deploy/.env.example`: 배포 디렉토리용 환경 변수 예시 파일
  * `api/src/config.rs` 모듈 (API 설정 로드)
  * Yew Frontend 설정 (API 엔드포인트 등):
    * `.env` 파일의 `FRONTEND_API_ENDPOINT` 값을 Docker 빌드 인자로 전달 (`--build-arg`).
    * `yew.Dockerfile` 빌드 단계에서 환경 변수로 설정 (`ENV`).
    * `yew/Trunk.toml`의 `[env]` 섹션을 통해 Yew 코드에서 `std::env!("FRONTEND_API_ENDPOINT")` 또는 `option_env!`로 접근.
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
  * Nginx 설정: `./deploy/nginx/nginx.conf.template` 및 `./deploy/nginx/docker-entrypoint.sh` 사용.
    * 역할: 리버스 프록시 (`/api/v1/` -> `api` 서비스), Yew 정적 파일 서빙 (`/` -> Yew 빌드 결과물), TLS 종료.
    * 프록시 헤더: 표준 헤더 설정 (`Host`, `X-Real-IP`, `X-Forwarded-For`, `X-Forwarded-Proto`).
    * 캐싱: 정적 에셋 (`/static/`) 캐싱 설정.
  * 볼륨 마운트:
    * Nginx 설정 템플릿: `./deploy/nginx/nginx.conf.template` -> `/etc/nginx/conf.d/default.conf.template` (읽기 전용).
    * Nginx 엔트리포인트 스크립트: `./deploy/nginx/docker-entrypoint.sh` -> `/docker-entrypoint.sh`.
    * TLS 인증서: `./deploy/secure/tls` -> `/etc/nginx/certs` (읽기 전용).
    * MongoDB 데이터: `mongo-data` (명명된 볼륨) -> `/data/db`.
  * TLS:
    * Nginx 레벨에서 TLS 종료 처리.
    * 수동 발급된 PEM 형식 인증서 사용 (`cert.pem`, `privkey.pem` 파일).
    * 인증서 경로 설정: Docker 볼륨 마운트 (`/etc/nginx/certs`) 및 Nginx 설정 파일에서 직접 경로 참조.
    * (개발/테스트 시 자체 서명 인증서 생성 스크립트 `./deploy/gen_certs.sh` 활용 가능).

# 구현 명세

## 1. 프로젝트 구조

```
frand-api/
├── .env                # 공용 환경 변수 설정 파일 (API, Docker, DB, Yew 빌드 시 사용)
├── .env.test           # 테스트 환경 변수 설정 파일
├── .env.example        # 환경 변수 예시 파일
├── .gitignore
├── .dockerignore       # Docker 빌드 시 무시 파일 목록
├── Cargo.toml          # 워크스페이스 설정 파일 (members = ["api", "yew"])
├── api/                # API 서버 패키지
│   ├── Cargo.toml      # mongodb 의존성 제거됨
│   ├── src/            # config.rs 에서 DB 연결 정보 처리, root 핸들러 제거됨
│   └── tests/          # root 테스트 제거됨
├── yew/                # Yew Frontend 패키지
│   ├── Cargo.toml
│   ├── Trunk.toml      # [env] 섹션에 FRONTEND_API_ENDPOINT 정의
│   ├── index.html      # Yew 앱 기본 HTML
│   ├── static/         # 정적 에셋 (예: style.css) - Trunk 가 빌드 시 포함
│   │   └── style.css
│   └── src/            # std::env!("FRONTEND_API_ENDPOINT") 사용
│       ├── lib.rs
│       └── main.rs
├── deploy/             # 배포 관련 파일
│   ├── .env.example    # 배포 디렉토리용 환경 변수 예시
│   ├── docker-compose.yml # api, yew_frontend, mongo 서비스 정의
│   ├── api.Dockerfile     # API 서비스 Dockerfile (debian:bookworm-slim 기반)
│   ├── yew.Dockerfile     # Yew Frontend 서비스 Dockerfile (nginx:1.27.4-alpine-slim 기반)
│   ├── gen_certs.sh    # 자체 서명 인증서 생성 스크립트
│   ├── nginx/
│   │   ├── nginx.conf.template # Nginx 설정 템플릿
│   │   └── docker-entrypoint.sh # Nginx 시작 스크립트
│   └── secure/
│       └── tls/          # TLS 인증서 저장 위치 (호스트)
│           ├── cert.pem
│           └── privkey.pem
└── README.md
```
* **참고:** Yew 빌드 결과물은 `./yew/dist`에 생성되며, `yew.Dockerfile`에서 Nginx 이미지 내부(`/usr/share/nginx/html`)로 복사됩니다. `yew/static/style.css`는 Trunk가 빌드 시 자동으로 포함시킵니다.

## 2. 주요 파일 역할 및 의존성

* **`api/Cargo.toml`**: `mongodb` 의존성 제거됨.
* **`api/src/config.rs`**: MongoDB 연결 정보(`DATABASE_USER`, `DATABASE_PASS`, `DATABASE_HOST`, `DATABASE_PORT`)를 환경 변수에서 읽어 `Config` 구조체에 포함하고, 연결 문자열 생성 로직 추가 (`mongodb_uri` 메서드). `ROCKET_PORT` 기본값 `8080`으로 변경.
* **`api/src/lib.rs`**: 루트 핸들러 마운트 제거.
* **`frand-api/.env`**: `DATABASE_USER`, `DATABASE_PASS` 변수 추가. `DATABASE_HOST`, `DATABASE_PORT`는 필요시 기본값 설정 가능 (docker-compose에서 오버라이드). `FRONTEND_API_ENDPOINT` 변수 포함.
* **`frand-api/.env.example`**: `.env` 파일의 예시 제공.
* **`frand-api/.dockerignore`**: Docker 빌드 컨텍스트에서 제외할 파일 및 디렉토리 목록 정의 (예: `target`).
* **`yew/Cargo.toml`**: Yew 패키지 의존성 정의.
* **`yew/Trunk.toml`**: `[env]` 섹션에 `FRONTEND_API_ENDPOINT = {}` 명시하여 빌드 시 환경 변수 주입 활성화. 개발 서버 프록시 설정 포함.
* **`yew/index.html`**: Yew 애플리케이션이 마운트될 기본 HTML 구조 정의. `static/style.css` 링크 포함.
* **`yew/src/lib.rs`**: Yew 애플리케이션의 주요 로직 (컴포넌트, 라우팅, API 호출 등) 구현. API 요청 시 `std::env!("FRONTEND_API_ENDPOINT")` 또는 `option_env!` 사용.
* **`yew/src/main.rs`**: Yew 애플리케이션 실행 진입점. `yew_frontend::run_app()` 호출.
* **`yew/static/style.css`**: Yew 애플리케이션의 기본 스타일 정의.
* **`deploy/docker-compose.yml`**: `mongo` 서비스 추가. `api` 서비스에 `DATABASE_HOST=mongo` 환경 변수 및 `mongo` 의존성 추가. `yew_frontend` 서비스(Nginx)의 TLS 볼륨 마운트 경로 수정 (`/etc/nginx/certs`). `mongo` 서비스에 인증 환경 변수 및 데이터 볼륨 설정 추가. `yew_frontend` 서비스의 Dockerfile 경로 수정. `api` 서비스 포트 노출 추가 (디버깅용). `yew_frontend` 서비스에 `nginx.conf.template` 및 `docker-entrypoint.sh` 볼륨 마운트 및 `command` 추가.
* **`deploy/api.Dockerfile`**: 실행 스테이지 베이스 이미지 `debian:bookworm-slim` 사용. `ca-certificates` 설치 확인. 바이너리 이름 `api` 확인. 멀티 스테이지 빌드 사용.
* **`deploy/yew.Dockerfile`**: 빌드 스테이지 베이스 이미지 `rust:1.86-slim` 사용. `ARG FRONTEND_API_ENDPOINT`, `ENV FRONTEND_API_ENDPOINT=$FRONTEND_API_ENDPOINT` 추가. 실행 스테이지 베이스 이미지 `nginx:1.27.4-alpine-slim` 사용. Yew 빌드 결과물 및 `static` 디렉토리를 Nginx 이미지로 복사.
* **`deploy/nginx/nginx.conf.template`**: TLS 인증서 경로를 `/etc/nginx/certs/cert.pem`, `/etc/nginx/certs/privkey.pem` 으로 수정. API 프록시 경로 `/api/v1/` 사용. 정적 파일 캐싱 설정 추가 (`/static/`). 환경 변수 플레이스홀더 사용.
* **`deploy/nginx/docker-entrypoint.sh`**: Nginx 컨테이너 시작 시 `envsubst` 를 사용하여 `nginx.conf.template` 에 환경 변수를 적용하고 `default.conf` 를 생성한 후 Nginx 를 실행.
* **`deploy/.env.example`**: `deploy` 디렉토리에서 사용할 수 있는 환경 변수 예시 제공 (루트 `.env.example`과 동일 내용).
* **`deploy/gen_certs.sh`**: 개발/테스트용 자체 서명 TLS 인증서 생성 스크립트.

## 3. 설정 파일 (`Cargo.toml`, `.env`, `docker-compose.yml`, `nginx.conf.template`, `Trunk.toml`)

* **`frand-api/Cargo.toml`**: 워크스페이스 레벨 설정.
    ```toml
    // filepath: /home/frand-nano/Projects/frand-api/Cargo.toml
    [workspace]
    members = ["api", "yew"]
    resolver = "2"

    [workspace.package]
    version = "0.1.0"
    edition = "2021"
    license = "MIT"
    authors = ["frand-nano <frand.nano@gmail.com>"]

    [workspace.dependencies]
    rocket = "0.5.1"
    tokio = { version = "1.37.0", features = ["full"] }
    serde = { version = "1.0.200", features = ["derive"] }
    serde_json = "1.0.117"
    dotenvy = "0.15.7"
    log = "0.4.21"
    simple_logger = "5.0.0"
    yew = "0.21"
    yew-router = "0.18"
    wasm-bindgen = "0.2.89"
    web-sys = { version = "0.3.66", features = ["HtmlElement"] }
    gloo = "0.10"
    wasm-bindgen-futures = "0.4.41"
    ```
* **`api/Cargo.toml`**:
    ```toml
    # filepath: /home/frand-nano/Projects/frand-api/api/Cargo.toml
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
    # mongodb 의존성 제거됨

    ```
* **`yew/Cargo.toml`**:
    ```toml
    # filepath: /home/frand-nano/Projects/frand-api/yew/Cargo.toml
    [package]
    name = "yew_frontend"
    version.workspace = true
    edition.workspace = true
    license.workspace = true
    authors.workspace = true
    publish = false

    [dependencies]
    yew = { workspace = true, features = ["csr"] }
    yew-router = { workspace = true }
    wasm-bindgen = { workspace = true }
    web-sys = { workspace = true }
    gloo = { workspace = true }
    serde = { workspace = true }
    serde_json = { workspace = true }
    log = { workspace = true }
    wasm-bindgen-futures = { workspace = true }
    ```
* **`.env.example` (루트)**:
    ```bash
    # filepath: /home/frand-nano/Projects/frand-api/.env.example
    # API Service
    ROCKET_ADDRESS=0.0.0.0
    ROCKET_PORT=8080
    LOG_LEVEL=info
    DATABASE_HOST=localhost # Docker Compose 사용 시 mongo 서비스 이름으로 오버라이드됨
    DATABASE_PORT=27017
    DATABASE_USER=root
    DATABASE_PASS=example_password

    # Yew Frontend Service (Nginx)
    NGINX_EXTERNAL_PORT=443
    NGINX_EXTERNAL_HTTP_PORT=80
    NGINX_SERVER_NAME=localhost

    # Yew Application Build Argument
    FRONTEND_API_ENDPOINT=/api/v1
    ```
* **`deploy/.env.example`**: (루트 `.env.example`과 유사하나 `DATABASE_HOST`가 `mongo`로 설정됨)
    ```bash
    # filepath: /home/frand-nano/Projects/frand-api/deploy/.env.example
    # API Service
    ROCKET_ADDRESS=0.0.0.0
    ROCKET_PORT=8080
    LOG_LEVEL=info
    DATABASE_HOST=mongo
    DATABASE_PORT=27017
    DATABASE_USER=root
    DATABASE_PASS=example_password

    # Yew Frontend Service (Nginx)
    NGINX_EXTERNAL_PORT=443
    NGINX_EXTERNAL_HTTP_PORT=80
    NGINX_SERVER_NAME=localhost

    # Yew Application Build Argument
    FRONTEND_API_ENDPOINT=/api/v1
    ```
* **`deploy/docker-compose.yml`**:
    ```yaml
    # filepath: /home/frand-nano/Projects/frand-api/deploy/docker-compose.yml
    version: '3.8'

    services:
      api:
        build:
          context: .. # 워크스페이스 루트 기준
          dockerfile: deploy/api.Dockerfile
        image: frand-api:latest # 이미지 이름 지정
        # container_name: frand_api_service # 필요시 주석 해제
        restart: unless-stopped
        env_file:
          - ./.env # deploy 디렉토리의 .env 파일 사용
        environment:
          # Docker 내부 네트워크에서 mongo 서비스 이름으로 접근하도록 호스트 오버라이드
          DATABASE_HOST: mongo
        ports:
          - "8080:8080" # API 서비스 포트 노출 (내부 디버깅용)
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
            # deploy 디렉토리의 .env 파일에서 값 가져오기
            - FRONTEND_API_ENDPOINT=${FRONTEND_API_ENDPOINT}
        image: frand-api-yew-nginx:latest # 이미지 이름 변경
        # container_name: frand_yew_nginx # 필요시 주석 해제
        restart: unless-stopped
        env_file:
          - ./.env # deploy 디렉토리의 .env 파일 사용
        ports:
          - "${NGINX_EXTERNAL_PORT}:443"
          - "${NGINX_EXTERNAL_HTTP_PORT}:80"
        volumes:
          - ./nginx/nginx.conf.template:/etc/nginx/conf.d/default.conf.template:ro
          - ./nginx/docker-entrypoint.sh:/docker-entrypoint.sh
          - ./secure/tls:/etc/nginx/certs:ro # TLS 인증서 마운트 (경로 수정)
          - /etc/localtime:/etc/localtime:ro
        depends_on:
          - api
        networks:
          - frand-api-network
        command: ["/bin/sh", "/docker-entrypoint.sh"]

      mongo:
        image: mongo:6.0 # 특정 버전 사용 권장
        # container_name: frand_mongo_db # 필요시 주석 해제
        restart: unless-stopped
        env_file:
          - ./.env # deploy 디렉토리의 .env 파일 사용
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
* **`deploy/nginx/nginx.conf.template`**:
    ```plaintext
    // filepath: /home/frand-nano/Projects/frand-api/deploy/nginx/nginx.conf.template
    server {
        listen ${NGINX_EXTERNAL_HTTP_PORT};
        server_name ${NGINX_SERVER_NAME};

        # HTTP -> HTTPS 리다이렉션
        location / {
            return 301 https://$host$request_uri;
        }
    }

    server {
        listen ${NGINX_EXTERNAL_PORT} ssl;
        http2 on;
        server_name ${NGINX_SERVER_NAME};

        # TLS 설정
        ssl_certificate /etc/nginx/certs/cert.pem;
        ssl_certificate_key /etc/nginx/certs/privkey.pem;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384;
        ssl_prefer_server_ciphers off;

        # API 프록시 설정
        location ${FRONTEND_API_ENDPOINT}/ {
            proxy_pass http://api:${ROCKET_PORT}${FRONTEND_API_ENDPOINT}/;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_read_timeout 90s;
        }

        # 정적 파일용 경로 추가
        location /static/ {
            alias /usr/share/nginx/static/;
            expires 7d;
            add_header Cache-Control "public, max-age=604800";
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
* **`deploy/nginx/docker-entrypoint.sh`**:
    ```bash
    #!/bin/sh
    # filepath: /home/frand-nano/Projects/frand-api/deploy/nginx/docker-entrypoint.sh
    set -e

    # nginx.conf 템플릿에 환경 변수 적용
    envsubst '${NGINX_SERVER_NAME} ${NGINX_EXTERNAL_PORT} ${NGINX_EXTERNAL_HTTP_PORT} ${ROCKET_PORT} ${FRONTEND_API_ENDPOINT} ${LOG_LEVEL}' < /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf

    # 환경 변수가 적용된 설정 파일 확인 (디버깅용)
    echo "Nginx 설정 파일이 생성되었습니다:"
    echo "-----------------------------------"
    cat /etc/nginx/conf.d/default.conf
    echo "-----------------------------------"

    # Nginx 실행
    exec nginx -g 'daemon off;'
    ```
* **`yew/Trunk.toml`**:
    ```toml
    # filepath: /home/frand-nano/Projects/frand-api/yew/Trunk.toml
    [build]
    target = "index.html" # 빌드 대상 HTML 파일 지정
    dist = "dist"
    public_url = "/"

    [env]
    # 빌드 시점에 환경 변수 주입 (Docker 빌드 시 --build-arg 로 전달됨)
    FRONTEND_API_ENDPOINT = {}

    [watch]
    # 파일 변경 감지 시 무시할 경로
    ignore = ["../target", "../deploy"]

    [serve]
    # Trunk 개발 서버 설정
    address = "127.0.0.1"
    port = 8080 # 개발 서버 포트 (API 서버와 충돌하지 않도록 주의)
    open = true # 개발 서버 시작 시 브라우저 자동 열기

    # 개발 서버 프록시 설정
    [[proxy]]
    rewrite = "/api/" # /api/ 경로 요청을 백엔드로 전달
    backend = "http://localhost:8080/api/" # 로컬 API 서버 주소 (api 패키지 실행 시)
    ```

## 4. 핵심 구조체 및 함수 시그니처

(API, Yew Frontend 및 MongoDB 연동 부분은 컴포넌트 및 데이터 모델 구현 후 필요시 추가)

## 5. API 엔드포인트

| 메소드 | 경로             | 설명                  | 요청 본문 | 응답 형식                 | 성공 상태 코드 | 실패 상태 코드 (Catcher) |
| :----- | :--------------- | :-------------------- | :-------- | :------------------------ | :------------- | :----------------------- |
| GET    | `/api/v1/health` | 서버 상태 확인 (헬스첵) | 없음      | `application/json` (`ApiResponse<HealthStatus>`) | 200 OK         | 404, 405 등 (ApiError)   |
* **참고:** `/` 엔드포인트는 제거되었습니다.

## 6. 오류 처리

(API 오류 처리는 `api/src/error.rs` 정의 및 Rocket catcher 사용)

## 7. 구현 가이드라인

* **Docker 빌드**: 각 Dockerfile은 멀티 스테이지 빌드를 활용하여 최종 이미지 크기를 최적화. `yew.Dockerfile` 빌드 시 `--build-arg`를 사용하여 `.env`의 값을 전달하고, Trunk가 이를 사용하도록 설정. 명시된 베이스 이미지 버전 사용 권장 (`rust:1.86-slim`, `nginx:1.27.4-alpine-slim`, `debian:bookworm-slim`). `.dockerignore` 파일을 사용하여 불필요한 파일이 빌드 컨텍스트에 포함되지 않도록 관리.
* **Docker Compose**: `depends_on`을 사용하여 서비스 시작 순서 제어 (`yew_frontend` -> `api` -> `mongo`). `restart: unless-stopped` 정책 사용. 볼륨 마운트는 필요한 설정, 보안 파일, 데이터 영속성(`mongo-data`)에 대해서만 사용 (읽기 전용 권장). 이미지 버전 고정 권장 (`mongo:6.0`). `env_file`을 사용하여 `deploy/.env` 파일의 환경 변수를 컨테이너에 주입.
* **Nginx**: TLS 종료, API 리버스 프록시, Yew Frontend 정적 파일 서빙 역할 수행. `try_files`를 사용하여 Yew 라우터 지원. TLS 설정은 마운트된 인증서 경로(`/etc/nginx/certs`) 직접 참조. `/static/` 경로에 대한 캐싱 설정 추가. `nginx.conf.template` 과 `docker-entrypoint.sh` 를 사용하여 환경 변수 기반 설정 적용.
* **Yew 빌드**: `Trunk`를 사용하여 Yew 애플리케이션 빌드. `Trunk.toml`에서 빌드 경로, public URL, 환경 변수 설정 관리. `yew/static` 폴더의 에셋은 Trunk가 자동으로 처리하여 `dist` 폴더에 포함시키고, `yew.Dockerfile`에서 Nginx 이미지의 `/usr/share/nginx/static/` 경로로 복사됨. `index.html` 파일은 `yew/` 디렉토리에 위치.
* **환경 변수 관리**: `.env` 파일을 중심으로 환경 변수 관리. API 서버는 `dotenvy`로 직접 로드, Docker Compose는 `env_file` 및 `environment`, `args`로 사용, Yew 앱은 Trunk의 `[env]` 기능을 통해 빌드 시점에 주입받음. `.env.example` 파일을 제공하여 필요한 환경 변수 목록 안내.
* **MongoDB 연동**: API 서버는 `mongodb` 드라이버를 사용하여 `mongo` 서비스에 연결. 연결 정보는 `Config` 구조체를 통해 관리 (`mongodb_uri` 메서드 활용).
* **정적 파일**: Yew 빌드 결과물(`dist` 폴더 내용)은 `yew.Dockerfile`에서 Nginx 이미지로 복사되어 서빙됨 (`/usr/share/nginx/html` 및 `/usr/share/nginx/static`).
* **Docker 이미지 환경**: API 및 Yew 빌드/실행에 필요한 시스템 패키지(`ca-certificates`, `pkg-config`, `libssl-dev`, `nodejs` 등)는 각 Dockerfile에 명시. 추가 필요시 구현 단계에서 식별하여 반영.
* **자체 서명 인증서 (개발용)**: 개발 또는 테스트 환경에서는 `./deploy/gen_certs.sh` 스크립트를 활용하여 `./deploy/secure/tls` 위치에 인증서를 생성하는 것을 고려할 수 있음.

