# API 서버 (`api` 패키지)

`frand-api` 워크스페이스의 핵심 REST API 서버입니다. Rocket 프레임워크를 기반으로 구현되었습니다.

## 주요 기술

*   Rust
*   Rocket (`0.5.1`)
*   Tokio (`1.37.0`)
*   Serde (JSON 처리)
*   dotenvy (환경 변수 관리)
*   log / simple_logger (로깅)
*   MongoDB Rust Driver (`mongodb 2.8.2`)
*   Chrono (날짜/시간 처리)
*   Validator (입력값 유효성 검사)
*   Futures (비동기 스트림 처리)

## 설정

API 서버 설정은 프로젝트 루트의 `.env` 파일을 통해 관리됩니다. 테스트 환경에서는 `.env.test` 파일이 사용됩니다.

필요한 주요 환경 변수는 다음과 같습니다.

*   `ROCKET_ADDRESS`: 서버 바인딩 주소 (Docker 내부에서는 `0.0.0.0` 사용 권장)
*   `ROCKET_PORT`: 서버 리스닝 포트 (기본값: `8080`)
*   `LOG_LEVEL`: 로깅 레벨 (기본값: `info`)
*   `DATABASE_USER`: MongoDB 사용자 이름
*   `DATABASE_PASS`: MongoDB 비밀번호
*   `DATABASE_HOST`: MongoDB 호스트 (Docker Compose 사용 시 `mongo` 서비스 이름으로 자동 설정됨)
*   `DATABASE_PORT`: MongoDB 포트 (기본값: `27017`)
*   `DATABASE_NAME`: 사용할 MongoDB 데이터베이스 이름 (기본값: `frand_api_db`, 테스트 시 `frand_api_db_test`)

루트 디렉토리에 `.env` 파일을 생성하고 필요한 값을 설정하십시오. 예시는 프로젝트 루트의 `.env.example` 파일 또는 `docs/spec_03.md` 문서를 참고하세요.

## 빌드

개별 빌드보다는 Docker를 통한 빌드를 권장합니다. Docker 빌드는 프로젝트 루트의 `deploy` 디렉토리에 있는 `api.Dockerfile`을 사용합니다.

```bash
# 워크스페이스 전체 빌드 (참고용)
cargo build --workspace

# Docker 이미지 빌드 (deploy 디렉토리에서 실행)
# docker-compose build api
```

## 실행 (Docker Compose 권장)

프로젝트 루트의 `deploy` 디렉토리에서 `docker-compose up -d` 명령어를 사용하여 전체 서비스(API, Frontend, DB)를 실행하는 것을 강력히 권장합니다.

```bash
# Docker Compose 로 실행 (deploy 디렉토리에서)
docker-compose up -d api # API 서비스만 실행 (다른 서비스 의존성 주의)
# 또는
docker-compose up -d # 전체 서비스 실행

# 개별 실행 (개발 시, .env 파일 필요)
# cd api
# cargo run
```

## 테스트

```bash
# api 패키지 디렉토리에서 실행
cargo test
```

테스트는 `.env.test` 파일의 설정을 사용합니다 (`DATABASE_NAME` 포함). 테스트 실행 전 루트 디렉토리에 `.env.test` 파일을 생성하고 필요에 따라 설정하십시오. `tests/health.rs` 와 `tests/memo.rs` 파일에서 테스트 케이스를 확인할 수 있습니다.

## 상세 정보

프로젝트의 자세한 설계 및 구현 명세는 프로젝트 루트의 [`docs/`](../../docs/) 디렉토리의 명세 문서들을 참고하십시오.
