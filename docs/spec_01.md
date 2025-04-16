# 정보 요약
* **프로그램 유형:** Rust 기반 REST API 서버 (`frand-api` 워크스페이스 구조)
* **주요 기술:** Rust, Rocket 프레임워크 (`0.5.1`), Tokio (`1.37.0`)
* **주요 기능:** 특정 기능 미정, 점진적 개발 예정. 초기 단계에서는 기본 설정 및 health check만 구현.
* **폴더 구조:** 표준 Rust 워크스페이스 구조 (`api` 패키지는 `main.rs`와 `lib.rs`를 모두 가짐)
* **데이터베이스:** MongoDB (`mongodb` 크레이트 사용, 연결 풀링 사용 예정 - 향후 구현)
  * 연결 정보: 환경 변수 사용 (`DATABASE_USER`, `DATABASE_PASS`, `DATABASE_HOST`, `DATABASE_PORT`) - 향후 구현 시 `config.rs`에서 조합하여 연결 문자열 생성
  * 데이터 스키마: 미정 (기능 미정)
* **설정 관리:** `.env` (기본), `.env.test` (테스트용) 파일과 `api/src/config.rs` 모듈 사용
  * `dotenvy` (`0.15.7`) 크레이트를 사용하여 `.env` 파일 로드 (`dotenvy::dotenv().ok();`)
  * `std::env::var()` 를 사용하여 환경 변수 읽기 및 `Config` 구조체에 수동 매핑 (`ROCKET_ADDRESS`, `ROCKET_PORT`, `LOG_LEVEL` 필드만 초기 구현)
  * Rocket managed state를 활용하여 핸들러에서 설정값 접근 (향후 필요시 구현)
  * 주요 환경 변수:
    * `ROCKET_ADDRESS`: 서버 바인딩 주소 (기본값: `0.0.0.0`)
    * `ROCKET_PORT`: 서버 리스닝 포트 (기본값: `8080`)
    * `LOG_LEVEL`: 로깅 레벨 (기본값: `info`, 값: `trace`, `debug`, `info`, `warn`, `error`)
    * `DATABASE_USER`: MongoDB 사용자 이름 (기본값: 없음) - 향후 사용
    * `DATABASE_PASS`: MongoDB 비밀번호 (기본값: 없음) - 향후 사용
    * `DATABASE_HOST`: MongoDB 호스트 (기본값: `localhost`) - 향후 사용
    * `DATABASE_PORT`: MongoDB 포트 (기본값: `27017`) - 향후 사용
* **API 버전 관리:** URL 경로 사용 (예: `/api/v1/`)
* **데이터 형식:** JSON (요청/응답)
* **API 응답 구조:** `api/src/response.rs` 및 `api/src/error.rs` 에 정의된 표준 구조 사용
  * 성공: `ApiResponse<T>: { success: true, data: T }` (T: `serde::Serialize`)
  * 실패: `ApiError: { success: false, error: { code: String, message: String, details: Option<serde_json::Value> }, status: rocket::http::Status }` (`Responder` 구현)
    * `details` 필드: 주로 유효성 검사 실패 시 필드별 오류 정보 등 구조화된 추가 정보를 `serde_json::Value` 형태로 제공하는 데 사용. (예: `{"field_name": "error message"}`)
  * 오류 코드 (`String`, `SCREAMING_SNAKE_CASE`):
    * `BAD_REQUEST`: 잘못된 요청 (일반)
    * `UNAUTHORIZED`: 인증 필요
    * `FORBIDDEN`: 권한 없음
    * `NOT_FOUND`: 리소스 없음
    * `METHOD_NOT_ALLOWED`: 허용되지 않은 HTTP 메소드
    * `INTERNAL_SERVER_ERROR`: 서버 내부 오류
    * `VALIDATION_ERROR`: 입력값 유효성 검사 실패 (주로 `details` 필드와 함께 사용)
    * (기능 추가 시 필요한 오류 코드 추가 정의)
* **오류 처리:** Rocket catcher 를 구현하여 프레임워크 기본 오류(404, 405, 500)를 `ApiError` 형식으로 변환하여 응답 일관성 유지.
* **초기 구현 목표 (MVP):**
  * `/` 엔드포인트: `GET` 요청 시 "hello world" `text/plain` 응답 (상태 코드 200)
  * `/api/v1/health` 엔드포인트: `GET` 요청 시 표준 `ApiResponse<HealthStatus>` 사용, 성공 시 `data` 필드에 `HealthStatus { status: "ok".to_string() }` 포함 (상태 코드 200, `application/json`)
* **향후 구현 기능 (spec_02.md 에서 상세화):**
  * MongoDB 연동 (컬렉션 및 데이터 구조 미정) -> **spec_02.md 에서 구현**
  * 사용자 인증: Google OAuth 인증 -> JWT 인증 유지 -> API Key 발급 (향후 구현)
  * Docker 기반 배포 -> **spec_02.md 에서 구현**
  * 워크스페이스 내 추가 패키지: 프론트엔드 (Yew), WebSocket 서버 (향후 구현) -> **Yew Frontend 는 spec_02.md 에서 구현**
* **데이터 모델링:**
  * `serde` (`1.0.200`) 를 이용한 Rust 구조체 정의 (`api/src/models/` 또는 기능별 모듈 내 위치)
  * `HealthStatus { status: String }` 구조체는 `api/src/models/health.rs` 에 정의 (확장성 고려).
* **로깅:**
  * `log` (`0.4.21`) 크레이트 인터페이스 사용, 구현체로 `simple_logger` (`5.0.0`) 사용 (`api/src/logger.rs` 에서 초기화)
  * 로그 레벨은 `LOG_LEVEL` 환경 변수로 제어 (기본값 `info`)
  * 기본 텍스트 로그 포맷 사용 (추가 설정 없음, 타임스탬프 등 기본 제공 포맷 사용)
* **테스트:**
  * `api` 패키지 내 `tests/` 폴더에 통합 테스트 집중 (Rocket local dispatch 활용)
  * 테스트 파일은 엔드포인트 또는 기능 단위로 분리 (예: `tests/health.rs`, `tests/root.rs`)
  * 테스트 환경 설정을 위한 공통 로직은 `api/tests/common/mod.rs` 에 배치 (예: `.env.test` 로딩 함수 `setup()`)
  * `.env.test` 설정 파일 사용. 각 테스트 실행 전 `common::setup()` 호출하여 환경 설정.
  * 초기 테스트 시나리오:
    * `GET /`: 200 OK, 본문 "hello world", Content-Type `text/plain`
    * `POST /`: `ApiError` (코드: `METHOD_NOT_ALLOWED`, 상태 코드: 405) - Catcher 구현 확인
    * `GET /api/v1/health`: 200 OK, 본문 `{"success":true,"data":{"status":"ok"}}`, Content-Type `application/json`
    * `POST /api/v1/health`: `ApiError` (코드: `METHOD_NOT_ALLOWED`, 상태 코드: 405) - Catcher 구현 확인
  * 단위 테스트는 필요시 추가 (`api/src/` 내 모듈별 `tests` 서브모듈)
  * 테스트 전략 및 커버리지 목표: 향후 결정
* **프로젝트 설정:**
    * 워크스페이스 루트 `Cargo.toml`에서 공통 메타데이터 및 의존성 정의
  * `api` 패키지 `Cargo.toml`에서 워크스페이스 설정 상속 및 필요 의존성 추가
* **코드 스타일:** `rustfmt` 기본 설정 사용 (`cargo fmt` 실행)
* **의존성 관리:** 표준 `cargo` 명령어 사용 (`cargo update`, `cargo add`, `cargo rm`)
* **CI/CD:** GitHub Actions 사용 (향후 구축)
* **제약 조건:** 특정 성능 요구사항 없음

# 구현 명세

## 1. 프로젝트 구조 (MVP)

```
frand-api/
├── .env                # 개발 환경 변수 설정 파일
├── .env.test           # 테스트 환경 변수 설정 파일
├── .gitignore          # Git 무시 파일 목록
├── Cargo.toml          # 워크스페이스 설정 파일
├── api/                # API 서버 패키지
│   ├── Cargo.toml      # api 패키지 설정 파일
│   ├── src/            # 소스 코드 루트
│   │   ├── config.rs   # 설정 로딩 및 관리 모듈
│   │   ├── error.rs    # ApiError 정의 및 Catcher 구현 모듈
│   │   ├── handlers/   # 요청 핸들러 모듈
│   │   │   ├── mod.rs
│   │   │   ├── health.rs # /api/v1/health 엔드포인트 핸들러
│   │   │   └── root.rs   # / 엔드포인트 핸들러
│   │   ├── lib.rs      # 라이브러리 루트 (애플리케이션 빌더 등)
│   │   ├── logger.rs   # 로거 초기화 모듈
│   │   ├── main.rs     # 실행 진입점
│   │   ├── models/     # 데이터 모델 정의 모듈
│   │   │   ├── mod.rs
│   │   │   └── health.rs # HealthStatus 구조체 정의
│   │   └── response.rs # ApiResponse 정의 모듈
│   └── tests/          # 통합 테스트 루트
│       ├── common/     # 테스트 공통 모듈
│       │   └── mod.rs  # 테스트 셋업 함수 (setup) 등
│       ├── health.rs   # Health check 엔드포인트 테스트
│       └── root.rs     # Root 엔드포인트 테스트
└── README.md           # 프로젝트 설명 파일
```

## 2. 주요 파일 역할 및 의존성

* **`frand-api/Cargo.toml`**: 워크스페이스 정의, 공통 의존성 및 메타데이터 관리.
* **`frand-api/.env`**: 개발 환경 설정 (ROCKET_*, LOG_LEVEL 등).
* **`frand-api/.env.test`**: 테스트 환경 설정 (필요시 `.env` 와 다른 값 설정).
* **`frand-api/api/Cargo.toml`**: `api` 패키지 의존성 정의 (워크스페이스 의존성 상속), Rocket 기능 활성화 (`json`).
* **`api/src/main.rs`**: 프로그램 시작점. `dotenvy`, 로거, 설정 초기화 후 Rocket 인스턴스 빌드 및 실행 (`api::build_rocket().launch()`).
    * 의존성: `api`, `dotenvy`, `log`.
* **`api/src/lib.rs`**: `api` 패키지의 라이브러리 코드. Rocket 인스턴스 생성 및 설정 함수 (`build_rocket`) 포함. 핸들러, 라우트, catcher 등록.
    * 의존성: `rocket`, `crate::config`, `crate::handlers`, `crate::error`.
* **`api/src/config.rs`**: `Config` 구조체 정의. 환경 변수 로딩 및 `Config` 인스턴스 생성 함수 (`load`) 구현.
    * 의존성: `std::env`.
* **`api/src/logger.rs`**: 로거 초기화 함수 (`init_logger`) 구현. `LOG_LEVEL` 환경 변수 기반 로깅 레벨 설정.
    * 의존성: `log`, `simple_logger`, `std::str::FromStr`.
* **`api/src/response.rs`**: `ApiResponse<T>` 구조체 및 관련 구현 정의. `serde::Serialize` 활용.
    * 의존성: `serde`.
* **`api/src/error.rs`**: `ApiError` 구조체 및 `Responder` 구현 정의. 표준 오류 코드 상수 정의. Rocket catcher 함수들 (`not_found`, `method_not_allowed`, `internal_error`) 구현.
    * 의존성: `serde`, `serde_json`, `rocket`.
* **`api/src/models/mod.rs`**: `models` 모듈 선언.
* **`api/src/models/health.rs`**: `HealthStatus` 구조체 정의 (`serde::Serialize` 파생).
    * 의존성: `serde`.
* **`api/src/handlers/mod.rs`**: `handlers` 모듈 선언 및 하위 핸들러 모듈 export.
* **`api/src/handlers/root.rs`**: `/` 경로의 `GET` 요청 처리 핸들러 (`root_handler`) 구현. `text/plain` 응답 반환.
    * 의존성: `rocket`.
* **`api/src/handlers/health.rs`**: `/api/v1/health` 경로의 `GET` 요청 처리 핸들러 (`health_check_handler`) 구현. `Json<ApiResponse<HealthStatus>>` 반환.
    * 의존성: `rocket`, `crate::response::ApiResponse`, `crate::models::health::HealthStatus`.
* **`api/tests/common/mod.rs`**: 테스트 공통 함수 정의. `setup()` 함수 (테스트 환경 변수 로딩 등) 구현.
    * 의존성: `dotenvy`.
* **`api/tests/root.rs`**: `/` 엔드포인트 통합 테스트. `common::setup()` 호출, Rocket local client 생성, 요청 전송 및 응답 검증.
    * 의존성: `rocket`, `api`, `crate::common`.
* **`api/tests/health.rs`**: `/api/v1/health` 엔드포인트 통합 테스트. `common::setup()` 호출, Rocket local client 생성, 요청 전송 및 응답 검증 (JSON 응답 구조 확인).
    * 의존성: `rocket`, `serde_json`, `api`, `crate::common`.

## 3. 설정 파일 (`Cargo.toml`, `.env`)

* **`frand-api/Cargo.toml`**:
    ```toml
    // filepath: /Cargo.toml
    [workspace]
    members = ["api"]

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
    ```
* **`frand-api/api/Cargo.toml`**:
    ```toml
    // filepath: /api/Cargo.toml
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

    ```
* **`.env` (예시)**:
    ```dotenv
    # filepath: /.env
    ROCKET_ADDRESS=0.0.0.0
    ROCKET_PORT=8080
    LOG_LEVEL=info
    # DATABASE_USER= # spec_02.md 에서 필수 설정으로 변경됨
    # DATABASE_PASS= # spec_02.md 에서 필수 설정으로 변경됨
    # DATABASE_HOST=localhost # spec_02.md 에서 Docker 환경 고려
    # DATABASE_PORT=27017
    ```
* **`.env.test` (예시)**:
    ```dotenv
    # filepath: /.env.test
    # 테스트 시 포트 충돌 방지 등 필요시 다른 값 설정
    ROCKET_PORT=8001
    LOG_LEVEL=debug
    ```

## 4. 핵심 구조체 및 함수 시그니처

| 구분         | 이름                 | 타입         | 설명                                                                 | 주요 필드/인자/반환값                                                                                                                               | 위치                  |
| :----------- | :------------------- | :----------- | :------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------- |
| **구조체** | `Config`             | Struct       | 애플리케이션 설정값 저장                                             | `rocket_address: String`, `rocket_port: u16`, `log_level: String` (향후 DB 관련 필드 추가)                                                              | `api/src/config.rs`   |
|              | `ApiResponse<T>`     | Generic Struct | 표준 성공 응답 형식                                                  | `success: bool` (true 고정), `data: T` (where `T: Serialize`)                                                                                       | `api/src/response.rs` |
|              | `ApiError`           | Struct       | 표준 실패 응답 형식, `Responder` 구현                                | `success: bool` (false 고정), `error: ErrorDetails`, `status: Status`                                                                               | `api/src/error.rs`    |
|              | `ErrorDetails`       | Struct       | `ApiError` 내 오류 상세 정보                                         | `code: String` (오류 코드), `message: String` (오류 메시지), `details: Option<Value>` (추가 정보)                                                     | `api/src/error.rs`    |
|              | `HealthStatus`       | Struct       | Health check 응답 데이터                                             | `status: String`                                                                                                                                    | `api/src/models/health.rs` |
| **함수** | `load`               | Function     | 환경 변수 로드 및 `Config` 인스턴스 생성                             | `()` -> `Config`                                                                                                                                    | `api/src/config.rs`   |
|              | `init_logger`        | Function     | 로거 초기화                                                          | `(log_level: &str)` -> `Result<(), SetLoggerError>`                                                                                                  | `api/src/logger.rs`   |
|              | `build_rocket`       | Function     | Rocket 인스턴스 생성, 설정, 라우팅, catcher 등록                     | `(config: Config)` -> `Rocket<Build>`                                                                                                               | `api/src/lib.rs`      |
|              | `root_handler`       | Async Function | `/` 경로 GET 요청 핸들러                                             | `()` -> `&'static str` (반환값: "hello world")                                                                                                      | `api/src/handlers/root.rs` |
|              | `health_check_handler` | Async Function | `/api/v1/health` 경로 GET 요청 핸들러                            | `()` -> `Json<ApiResponse<HealthStatus>>`                                                                                                           | `api/src/handlers/health.rs` |
|              | `not_found`          | Function     | 404 오류 Catcher                                                     | `(&Request)` -> `ApiError`                                                                                                                          | `api/src/error.rs`    |
|              | `method_not_allowed` | Function     | 405 오류 Catcher                                                     | `(&Request)` -> `ApiError`                                                                                                                          | `api/src/error.rs`    |
|              | `internal_error`     | Function     | 500 오류 Catcher                                                     | `()` -> `ApiError`                                                                                                                                  | `api/src/error.rs`    |
|              | `setup`              | Function     | 테스트 환경 설정 함수                                                | `()` -> `()` (내부적으로 `.env.test` 로드)                                                                                                        | `api/tests/common/mod.rs` |

## 5. API 엔드포인트 (MVP)

| 메소드 | 경로             | 설명                  | 요청 본문 | 응답 형식                 | 성공 상태 코드 | 실패 상태 코드 (Catcher) |
| :----- | :--------------- | :-------------------- | :-------- | :------------------------ | :------------- | :----------------------- |
| GET    | `/`              | 기본 환영 메시지      | 없음      | `text/plain`              | 200 OK         | 404, 405 등 (ApiError)   |
| GET    | `/api/v1/health` | 서버 상태 확인 (헬스첵) | 없음      | `application/json` (`ApiResponse<HealthStatus>`) | 200 OK         | 404, 405 등 (ApiError)   |

## 6. 오류 처리

* 모든 API 실패 응답은 `ApiError` 구조체를 사용하여 JSON 형식으로 반환.
* `ApiError` 는 `success: false`, 오류 코드(`code`), 메시지(`message`), 상세 정보(`details`), HTTP 상태 코드(`status`)를 포함.
* Rocket 의 `#[catch]` 매크로를 사용하여 기본 오류 핸들러(404 Not Found, 405 Method Not Allowed, 500 Internal Server Error)를 등록.
* Catcher 함수는 해당 HTTP 상태 코드에 맞는 `ApiError` 를 생성하여 반환.
    * 404 -> `code: NOT_FOUND`, `status: Status::NotFound`
    * 405 -> `code: METHOD_NOT_ALLOWED`, `status: Status::MethodNotAllowed`
    * 500 -> `code: INTERNAL_SERVER_ERROR`, `status: Status::InternalServerError`

## 7. 구현 가이드라인

* **TDD**: `tests/` 디렉토리에 통합 테스트를 우선 작성하고 이를 통과하도록 기능을 구현. 필요시 단위 테스트 추가.
* **환경 변수**: `.env` 파일을 통해 설정을 관리하고, `config.rs` 에서 로드하여 `Config` 구조체로 관리.
* **로깅**: `log` 크레이트 매크로 (`info!`, `warn!`, `error!` 등)를 사용하여 주요 이벤트 및 오류 로깅. 로거는 `main.rs` 에서 초기화.
* **코드 스타일**: `cargo fmt` 를 사용하여 코드 스타일 일관성 유지.
* **모듈화**: 기능별로 모듈 분리 (`handlers`, `models`, `config`, `response`, `error`, `logger`).
* **DRY 원칙**: 반복되는 로직은 함수나 모듈로 분리하여 재사용.
* **비동기**: Rocket 핸들러 및 관련 I/O 작업은 `async`/`await` 사용. `main` 함수는 `#[rocket::main]` 사용.