# API 서버 (`api` 패키지)

`frand-api` 워크스페이스의 핵심 REST API 서버입니다. Rocket 프레임워크를 기반으로 구현되었습니다.

## 주요 기술

*   Rust
*   Rocket (`0.5.1`)
*   Tokio (`1.37.0`)
*   Serde (JSON 처리)
*   dotenvy (환경 변수 관리)
*   log / simple_logger (로깅)
*   MongoDB Rust Driver (`mongodb`)

## 설정

API 서버 설정은 프로젝트 루트의 `.env` 파일을 통해 관리됩니다. 테스트 환경에서는 `.env.test` 파일이 사용됩니다.

필요한 주요 환경 변수는 다음과 같습니다.

*   `ROCKET_ADDRESS`: 서버 바인딩 주소 (Docker 내부에서는 `0.0.0.0` 사용 권장)
*   `API_INTERNAL_PORT`: 서버 리스닝 포트 (기본값: `8000`)
*   `LOG_LEVEL`: 로깅 레벨 (기본값: `info`)
*   `DATABASE_USER`: MongoDB 사용자 이름
*   `DATABASE_PASS`: MongoDB 비밀번호
*   `DATABASE_HOST`: MongoDB 호스트 (Docker Compose 사용 시 `mongo` 서비스 이름으로 자동 설정됨)
*   `DATABASE_PORT`: MongoDB 포트 (기본값: `27017`)

루트 디렉토리에 `.env` 파일을 생성하고 필요한 값을 설정하십시오. 예시는 `docs/spec_02.md` 문서를 참고하세요.

## 빌드

개별 빌드보다는 Docker를 통한 빌드를 권장합니다.
```bash
# 워크스페이스 전체 빌드 (참고용)
cargo build --workspace
```

## 실행 (Docker Compose 권장)

프로젝트 루트의 `deploy` 디렉토리에서 `docker-compose up -d` 명령어를 사용하여 전체 서비스를 실행하는 것을 권장합니다.

```bash
# 개별 실행 (개발 시, .env 파일 필요)
cargo run
```

## 테스트

```bash
cargo test
```

테스트는 `.env.test` 파일의 설정을 사용합니다. 테스트 실행 전 루트 디렉토리에 `.env.test` 파일을 생성하고 필요에 따라 설정하십시오.

## 상세 정보

API 서버 및 전체 시스템의 자세한 설계 및 구현 명세는 프로젝트 루트의 [docs/spec_02.md](../../docs/spec_02.md) 파일을 참고하십시오. 초기 MVP 명세는 [docs/spec_01.md](../../docs/spec_01.md)에서 확인할 수 있습니다.
