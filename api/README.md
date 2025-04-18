# API 서버 (frand-api)

Rust와 Rocket 프레임워크를 사용하여 구축된 API 서버입니다.

## 기술 스택

*   **언어:** Rust
*   **웹 프레임워크:** Rocket (JSON 지원 포함)
*   **로깅:** `log`, `simple_logger`
*   **오류 처리:** `anyhow`, `thiserror`
*   **설정 관리:** `config`

자세한 기술 명세는 프로젝트 루트의 [`../docs/spec.md`](../docs/spec.md) 파일을 참고하세요.

## 프로젝트 구조

`src` 디렉토리 내부는 다음과 같은 모듈 구조를 따릅니다.

*   `config`: 애플리케이션 설정 로드 및 관리
*   `routes`: API 엔드포인트 정의
*   `handlers`: 요청 처리 로직 구현 (향후 사용 예정)
*   `models`: 데이터 구조 정의 (향후 사용 예정)
*   `services`: 비즈니스 로직 구현 (향후 사용 예정)

## 기능

*   `/api/v1/health`: 서버 상태 확인 엔드포인트. JSON 형식으로 상태 코드(200)와 현재 애플리케이션 버전을 반환합니다.

## API 버전 관리

URL 경로에 버전을 포함하는 방식을 사용합니다. API 버전 경로는 `config/*.toml` 파일의 `server.api_version` 항목에서 설정합니다. (예: `/api/v1/...`)

## 설정

애플리케이션 설정은 프로젝트 루트의 `config` 디렉토리에 위치한 TOML 파일을 통해 관리됩니다.

*   `config/default.toml`: 기본 설정 파일
    *   `log_level`: 애플리케이션 로그 레벨 (예: "info", "debug")
    *   `server.port`: 서버가 리스닝할 포트
    *   `server.host`: 서버 호스트 주소
    *   `server.api_version`: API 기본 경로 (예: "/api/v1")
*   `config/test.toml`: 테스트 환경 설정 파일 (기본 설정과 동일한 구조)

## 실행 방법

1.  Rust 개발 환경을 설정합니다.
2.  프로젝트 루트 디렉토리에서 다음 명령어를 실행합니다.

    ```bash
    cargo run --bin api
    ```
    *   기본적으로 `config/default.toml` 설정을 사용합니다.

## 테스트

통합 테스트 위주로 진행하며, Rocket의 `LocalClient`를 활용합니다. 테스트 실행 명령어는 다음과 같습니다.

```bash
cargo test --package api
```
*   테스트 시에는 `config/test.toml` 설정을 사용합니다.

## 향후 계획

*   MongoDB 연동 (`mongodb` 드라이버, `serde` 사용)
*   인증/인가 구현 (Google OAuth, JWT)
*   `User` 모델 등 데이터 모델 정의
*   GitHub Actions를 이용한 CI/CD 구축

자세한 내용은 [`../docs/spec.md`](../docs/spec.md) 파일을 참고하세요.
