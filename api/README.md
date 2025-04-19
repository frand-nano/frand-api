# API 서버 (frand-api)

Rust와 Rocket 프레임워크를 사용하여 구축된 API 서버입니다.

## 기술 스택

*   **언어:** Rust
*   **웹 프레임워크:** Rocket (JSON 지원 포함)
*   **데이터베이스:** MongoDB (`mongodb` 드라이버 사용)
*   **데이터 직렬화:** `serde`, `serde_json`
*   **로깅:** `log`, `simple_logger`
*   **오류 처리:** `anyhow`, `thiserror`
*   **설정 관리:** `dotenvy` (환경 변수 로딩)
*   **공통 코드:** `frand-api-common` (워크스페이스 내)

자세한 기술 명세는 프로젝트 루트의 [`../docs/spec.md`](../docs/spec.md) 파일을 참고하세요.

## 프로젝트 구조

`src` 디렉토리 내부는 다음과 같은 모듈 구조를 따릅니다.

*   `config`: 애플리케이션 설정 로드 및 관리 (`.env` 파일 사용)
*   `mongodb`: MongoDB 연결 및 관련 유틸리티 (Fairing 포함)
*   `models`: 데이터베이스 스키마 및 내부 데이터 구조 정의 (`User` 등)
*   `routes`: API 엔드포인트 정의 (`health`, `users` 등)
*   `error`: 사용자 정의 오류 타입 (`ApiError`) 및 응답 처리
*   `handlers`: (향후 사용 예정) 요청 처리 로직 구현
*   `services`: (향후 사용 예정) 비즈니스 로직 구현

## 기능

*   `/health`: 서버 상태 확인 엔드포인트. JSON 형식으로 상태 코드(200)와 현재 애플리케이션 버전을 반환합니다.
*   `/users`: 사용자 정보 CRUD 엔드포인트
    *   `GET /users`: 모든 사용자 목록 조회
    *   `POST /users`: 새 사용자 생성
    *   `GET /users/{user_id}`: 특정 사용자 정보 조회
    *   `DELETE /users/{user_id}`: 특정 사용자 삭제

API 엔드포인트 기본 경로는 설정 가능합니다. (예: `/api/v1`)

## API 버전 관리

URL 경로에 버전을 포함하는 방식을 사용합니다. API 기본 경로는 프로젝트 루트의 `.env` 파일의 `ROCKET_API_ENDPOINT` 환경 변수에서 설정합니다. (예: `/api/v1`)

## 설정

애플리케이션 설정은 프로젝트 루트의 `.env` 파일을 통해 관리됩니다. `dotenvy` 크레이트를 사용하여 이 파일을 로드합니다.

*   `.env` 파일 예시 (`deploy/.env.example` 또는 프로젝트 루트의 `.env.example` 참고):
    ```dotenv
    # API Service
    LOG_LEVEL=info
    ROCKET_ADDRESS=0.0.0.0
    ROCKET_PORT=8080
    ROCKET_API_ENDPOINT=/api/v1

    # MongoDB 연결 설정
    MONGO_HOST=localhost
    MONGO_PORT=27017
    MONGO_DB_NAME=frand_api_db
    MONGO_USER=root
    MONGO_PASSWORD=example_password

    # NGINX Service (deploy/.env 에서 사용)
    NGINX_HTTP_PORT=80
    NGINX_HTTPS_PORT=443
    ```
*   **주요 환경 변수:**
    *   `LOG_LEVEL`: 애플리케이션 로그 레벨 (예: "info", "debug")
    *   `ROCKET_ADDRESS`: 서버가 바인딩할 주소
    *   `ROCKET_PORT`: 서버가 리스닝할 포트
    *   `ROCKET_API_ENDPOINT`: API 기본 경로
    *   `MONGO_HOST`, `MONGO_PORT`, `MONGO_DB_NAME`, `MONGO_USER`, `MONGO_PASSWORD`: MongoDB 연결 정보
    *   `NGINX_HTTP_PORT`, `NGINX_HTTPS_PORT`: Nginx 서비스에서 사용할 포트 (Docker Compose에서 사용)

**주의:** 테스트 실행 시에는 프로젝트 루트의 `.env` 파일을 사용합니다. 테스트 환경을 위한 별도의 설정이 필요하다면 테스트 코드 내에서 환경 변수를 설정하거나 별도의 테스트용 `.env` 파일을 로드하도록 수정해야 합니다. (`api/tests/test_util.rs` 참고)

## 실행 방법

### 로컬 개발 환경

1.  Rust 개발 환경 및 MongoDB 서버를 설정합니다.
2.  프로젝트 루트 디렉토리에 `.env` 파일을 생성하고 필요한 환경 변수(MongoDB 접속 정보 포함)를 설정합니다.
3.  프로젝트 루트 디렉토리에서 다음 명령어를 실행합니다.

    ```bash
    cargo run --package api
    ```
    *   기본적으로 프로젝트 루트의 `.env` 설정을 사용합니다.

### Docker 사용

프로젝트 루트의 `deploy` 디렉토리에 있는 Docker 관련 파일을 사용합니다. 이 방식은 API 서버, MongoDB, Nginx 리버스 프록시를 함께 실행합니다. 자세한 내용은 [`../deploy/README.md`](../deploy/README.md) 파일을 참고하세요.

1.  `deploy/.env` 파일을 설정합니다. (MongoDB 접속 정보 포함)
2.  (최초 실행 시) `deploy/certs` 디렉토리에서 `./gen_certs.sh` 스크립트를 실행하여 로컬 테스트용 TLS 인증서를 생성합니다.
3.  `deploy` 디렉토리에서 다음 명령어를 실행합니다.
    ```bash
    docker-compose up -d --build
    ```

## 테스트

통합 테스트 위주로 진행하며, Rocket의 `LocalClient`를 활용합니다. MongoDB를 사용하는 테스트는 순차적으로 실행되어야 하므로 `serial_test` 크레이트를 사용합니다.

*   테스트 실행 명령어:
    ```bash
    cargo test --package frand-api
    ```
*   테스트 파일 구조 (`api/tests/`):
    *   `test_util.rs`: 테스트 환경 설정 (Rocket 클라이언트, 설정 로드) 헬퍼 함수
    *   `test_health.rs`: `/health` 엔드포인트 테스트
    *   `test_users.rs`: `/users` CRUD 엔드포인트 테스트 (`#[serial]` 어트리뷰트 사용)
*   테스트 시에는 프로젝트 루트의 `.env` 파일을 로드하여 설정을 사용합니다. (`api/tests/test_util.rs`에서 `../.env` 로드)

## 향후 계획

*   인증/인가 구현 (Google OAuth, JWT)
*   추가 데이터 모델 정의 (예: 친구 관계, 일정 등)
*   GitHub Actions를 이용한 CI/CD 구축

자세한 내용은 [`../docs/spec.md`](../docs/spec.md) 파일을 참고하세요.
