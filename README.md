# frand-api

Rust 기반 REST API 서버 프로젝트 (`frand-api`)입니다.

## 프로젝트 구조

본 프로젝트는 Rust 워크스페이스로 구성되어 있으며, 현재는 `api` 패키지만 포함되어 있습니다.

*   `api/`: 핵심 REST API 서버 구현체입니다. Rocket 프레임워크를 사용합니다.
*   `docs/`: 프로젝트 관련 문서가 위치합니다.

## 실행 방법

1.  **API 서버 실행:**
    ```bash
    cd api
    cargo run
    ```
    서버 실행 전에 `api/.env` 파일을 필요에 따라 설정하십시오. 자세한 내용은 `api/README.md` 파일을 참고하세요.

2.  **테스트 실행:**
    ```bash
    cd api
    cargo test
    ```

## 상세 정보

프로젝트의 자세한 설계 및 구현 명세는 [docs/spec_01.md](docs/spec_01.md) 파일을 참고하십시오.
