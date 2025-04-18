# API 서버 (frand-api)

Rust와 Rocket 프레임워크를 사용하여 구축된 API 서버입니다.

## 기술 스택

*   **언어:** Rust
*   **웹 프레임워크:** Rocket (JSON 지원 포함)
*   **로깅:** `log`, `simple_logger`
*   **오류 처리:** `anyhow`, `thiserror`
*   **설정 관리:** `dotenvy` (환경 변수 로딩)

자세한 기술 명세는 프로젝트 루트의 [`../docs/spec.md`](../docs/spec.md) 파일을 참고하세요.

## 프로젝트 구조

`src` 디렉토리 내부는 다음과 같은 모듈 구조를 따릅니다.

*   `config`: 애플리케이션 설정 로드 및 관리 (`.env` 파일 사용)
*   `routes`: API 엔드포인트 정의
*   `handlers`: 요청 처리 로직 구현 (향후 사용 예정)
*   `models`: 데이터 구조 정의 (향후 사용 예정)
*   `services`: 비즈니스 로직 구현 (향후 사용 예정)

## 기능

*   `/api/v1/health`: 서버 상태 확인 엔드포인트. JSON 형식으로 상태 코드(200)와 현재 애플리케이션 버전을 반환합니다. (API 엔드포인트 경로는 설정 가능)

## API 버전 관리

URL 경로에 버전을 포함하는 방식을 사용합니다. API 기본 경로는 프로젝트 루트의 `.env` 파일의 `ROCKET_API_ENDPOINT` 환경 변수에서 설정합니다. (예: `/api/v1`)

## 설정

애플리케이션 설정은 프로젝트 루트의 `.env` 파일을 통해 관리됩니다. `dotenvy` 크레이트를 사용하여 이 파일을 로드합니다.

*   `.env` 파일 예시:
    ```dotenv
    # API Service
    LOG_LEVEL=info
    ROCKET_ADDRESS=0.0.0.0
    ROCKET_PORT=8080
    ROCKET_API_ENDPOINT=/api/v1
    ```
*   **주요 환경 변수:**
    *   `LOG_LEVEL`: 애플리케이션 로그 레벨 (예: "info", "debug")
    *   `ROCKET_ADDRESS`: 서버가 바인딩할 주소
    *   `ROCKET_PORT`: 서버가 리스닝할 포트
    *   `ROCKET_API_ENDPOINT`: API 기본 경로

**주의:** 테스트 실행 시에는 프로젝트 루트의 `.env` 파일을 사용합니다. 테스트 환경을 위한 별도의 설정이 필요하다면 테스트 코드 내에서 환경 변수를 설정하거나 별도의 테스트용 `.env` 파일을 로드하도록 수정해야 합니다. (`api/tests/test_util.rs` 참고)

## 실행 방법

### 로컬 개발 환경

1.  Rust 개발 환경을 설정합니다.
2.  프로젝트 루트 디렉토리에 `.env` 파일을 생성하고 필요한 환경 변수를 설정합니다.
3.  프로젝트 루트 디렉토리에서 다음 명령어를 실행합니다.

    ```bash
    cargo run --package api
    ```
    *   기본적으로 프로젝트 루트의 `.env` 설정을 사용합니다.

### Docker 사용

프로젝트 루트의 `deploy` 디렉토리에 있는 Docker 관련 파일을 사용합니다. 자세한 내용은 [`../deploy/README.md`](../deploy/README.md) 파일을 참고하세요.

1.  `deploy/.env` 파일을 설정합니다.
2.  `deploy` 디렉토리에서 다음 명령어를 실행합니다.
    ```bash
    docker-compose up -d --build
    ```

## 테스트

통합 테스트 위주로 진행하며, Rocket의 `LocalClient`를 활용합니다. 테스트 실행 명령어는 다음과 같습니다.

```bash
cargo test --package api
```
*   테스트 시에는 프로젝트 루트의 `.env` 파일을 로드하여 설정을 사용합니다. (`api/tests/test_util.rs`에서 `../.env` 로드)

## 향후 계획

*   MongoDB 연동 (`mongodb` 드라이버, `serde` 사용)
*   인증/인가 구현 (Google OAuth, JWT)
*   `User` 모델 등 데이터 모델 정의
*   GitHub Actions를 이용한 CI/CD 구축

자세한 내용은 [`../docs/spec.md`](../docs/spec.md) 파일을 참고하세요.
