# API 서버 (frand-api)

Rust와 Rocket 프레임워크를 사용하여 구축된 API 서버입니다.

## 기술 스택

*   **언어:** Rust
*   **웹 프레임워크:** Rocket
*   **로깅:** `log`, `simple_logger`
*   **오류 처리:** `anyhow`, `thiserror`
*   **설정 관리:** `config`

자세한 기술 명세는 프로젝트 루트의 [`../docs/spec.md`](../docs/spec.md) 파일을 참고하세요.

## 프로젝트 구조

`src` 디렉토리 내부는 다음과 같은 모듈 구조를 따릅니다.

*   `routes`: API 엔드포인트 정의
*   `handlers`: 요청 처리 로직 구현
*   `models`: 데이터 구조 정의 (향후 사용 예정)
*   `services`: 비즈니스 로직 구현 (향후 사용 예정)

## 기능

*   `/health`: 서버 상태 확인 엔드포인트 (현재는 단순 상태 코드 반환)

## API 버전 관리

URL 경로에 버전을 포함하는 방식을 사용합니다. (예: `/api/v1/...`)

## 실행 방법

1.  Rust 개발 환경을 설정합니다.
2.  프로젝트 루트 디렉토리에서 다음 명령어를 실행합니다.

    ```bash
    cargo run
    ```

## 테스트

통합 테스트 위주로 진행하며, Rocket의 `LocalClient`를 활용합니다. 테스트 실행 명령어는 다음과 같습니다.

```bash
cargo test
```

## 향후 계획

*   MongoDB 연동 (`mongodb` 드라이버, `serde` 사용)
*   인증/인가 구현 (Google OAuth, JWT)
*   `User` 모델 등 데이터 모델 정의
*   GitHub Actions를 이용한 CI/CD 구축

자세한 내용은 [`../docs/spec.md`](../docs/spec.md) 파일을 참고하세요.
