# API 서버 (`api` 패키지)

`frand-api` 워크스페이스의 핵심 REST API 서버입니다. Rocket 프레임워크를 기반으로 구현되었습니다.

## 주요 기술

*   Rust
*   Rocket (`0.5.1`)
*   Tokio (`1.37.0`)
*   Serde (JSON 처리)
*   dotenvy (환경 변수 관리)
*   log / simple_logger (로깅)

## 설정

API 서버 설정은 프로젝트 루트의 `.env` 파일을 통해 관리됩니다. 테스트 환경에서는 `.env.test` 파일이 사용됩니다.

필요한 환경 변수는 다음과 같습니다.

*   `ROCKET_ADDRESS`: 서버 바인딩 주소 (기본값: `0.0.0.0`)
*   `ROCKET_PORT`: 서버 리스닝 포트 (기본값: `8000`)
*   `LOG_LEVEL`: 로깅 레벨 (기본값: `info`)
*   (향후 DB 관련 변수 추가 예정)

루트 디렉토리에 `.env` 파일을 생성하고 필요한 값을 설정하십시오. 예시:

```dotenv
# .env
ROCKET_ADDRESS=127.0.0.1
ROCKET_PORT=8080
LOG_LEVEL=debug
```

## 빌드

```bash
cargo build
```

## 실행

```bash
cargo run
```

서버는 `.env` 파일에 설정된 주소와 포트에서 실행됩니다.

## 테스트

```bash
cargo test
```

테스트는 `.env.test` 파일의 설정을 사용합니다. 테스트 실행 전 루트 디렉토리에 `.env.test` 파일을 생성하고 필요에 따라 설정하십시오.

## 상세 정보

API 서버의 자세한 설계 및 구현 명세는 프로젝트 루트의 [docs/spec_01.md](../../docs/spec_01.md) 파일을 참고하십시오.
