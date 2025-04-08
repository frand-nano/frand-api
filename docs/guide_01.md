# 정보 요약
* **프로그래밍 언어:** Rust
* **프로젝트 구조:** Cargo 워크스페이스 (`./`, `./api/`)
* **개발 접근 방식:** 테스트 주도 개발(TDD), 점진적 기능 확장
* **문서화:** 각 개발 단계를 `./docs/` 디렉토리에 Markdown 파일로 기록 (첫 단계: `guide_01.md`)
* **설정 관리:** `api/.env` (개발), `api/.env.test` (테스트) 파일 및 `dotenvy` 크레이트 사용
* **패키지 메타데이터:** 워크스페이스 레벨(`[workspace.package]`)에서 버전, 에디션, 라이선스, 작성자 정보 정의 및 상속
* **주요 기능 (API):** DB 정보 제공 및 연산 대행 (상세 내용 추후 구체화)
* **주요 프레임워크/라이브러리 (`api`):** Rocket, `log`, `simple_logger`, `serde`, `dotenvy`
* **API 패키지 구조:**
  * `api/src/lib.rs`에 `create_rocket` 함수 정의 (테스트 용이성)
  * 초기 구조에 `services`, `config.rs`, `tests/common` 포함
* **초기 구현 목표 (`api`):** `GET /` -> 단순 텍스트 'hello world' 응답
* **테스트 방식 (`api`):** `api/tests/` 디렉토리 내 통합 테스트 작성 (`lib::create_rocket`, `tests/common` 활용, `.env.test` 설정 사용)

# 구현 가이드
## 1. 프로젝트 초기 설정 (워크스페이스)
  * 루트 `Cargo.toml` 파일 생성:
    - 워크스페이스 정의 (`[workspace]`) 및 `api` 멤버 포함
    - 공통 패키지 메타데이터 정의 (`[workspace.package]`: version, edition, license, authors)
  * `api` 패키지 생성: 라이브러리 형태로 생성
  * 루트 `.gitignore` 파일 설정 (Rust 표준 `.gitignore` 내용 활용, `.env*` 포함 확인)
  * `api/.env.example`, `api/.env.test.example` 파일 생성 (설정 예시 제공)

## 2. `api` 패키지 기본 구조 (확정)
  * `api/src/`: 소스 코드 루트
    - `lib.rs`: 라이브러리 루트, Rocket 인스턴스 생성 함수 `create_rocket` 정의
    - `main.rs`: 애플리케이션 진입점, `.env` 로드, `api::create_rocket` 호출 및 실행, 로거 설정
    - `routes/`: API 라우트 핸들러 모듈화
      - `mod.rs`
      - `root.rs`: `GET /` 엔드포인트 핸들러 ("hello world")
      - `...` (기능별 라우트 모듈 추가 예정)
    - `models/`: 데이터 구조(struct) 정의 (추후 사용)
    - `services/`: 비즈니스 로직 분리
      - `mod.rs` (초기 생성)
    - `config.rs`: 애플리케이션 설정 관리 (환경 변수 로드 및 구조체 정의)
  * `api/tests/`: 통합 테스트 코드
    - `common/`: 테스트 헬퍼 함수 모듈 (테스트 시작 시 `.env.test` 로드)
      - `mod.rs` (초기 생성)
    - `root_test.rs`: `GET /` 엔드포인트 테스트 (`lib::create_rocket` 활용)
    - `...` (기능별 테스트 파일 추가 예정)
  * `api/.env`: 개발 환경 설정 파일 (Git 추적 안 함)
  * `api/.env.test`: 테스트 환경 설정 파일 (Git 추적 안 함)
  * `api/.env.example`: 개발 환경 설정 예시 파일 (Git 추적)
  * `api/.env.test.example`: 테스트 환경 설정 예시 파일 (Git 추적)

## 3. 초기 구현 사항 (`api` 패키지 - TDD 접근)
  * **Setup:** 빈 파일/디렉토리 생성 및 `.env` 파일 설정
    - `api/src/services/mod.rs`
    - `api/src/config.rs` (환경 변수 로드 로직 추가)
    - `api/tests/common/mod.rs` (테스트 환경 설정 로드 함수 추가)
    - `api/.env`, `api/.env.test` 파일 생성 및 기본 설정 추가 (예: `ROCKET_PORT=8000`)
  * **Test:** `api/tests/root_test.rs` 작성
    - 테스트 시작 전 `dotenvy::from_filename(".env.test").ok()` 호출 (또는 `tests/common` 활용)
    - `api::create_rocket()` 호출하여 테스트용 Rocket 인스턴스 생성
    - `rocket::local::blocking::Client::tracked()` 로 테스트 클라이언트 생성
    - 클라이언트의 `get("/")` 메서드로 요청 전송
    - 예상 응답(상태 코드 200 OK, 본문 "hello world") 검증 (이 단계에서는 테스트 실패)
  * **Code:** `api/src/lib.rs` 구현
    - `create_rocket()` 함수 정의: Rocket 인스턴스 빌드, `routes::root` 모듈의 라우트 마운트
    - 필요한 모듈(routes, services, config, models) 선언
  * **Code:** `api/src/routes/mod.rs` 및 `api/src/routes/root.rs` 구현
    - `root.rs`에 `hello()` 함수 정의 (`&'static str` "hello world" 반환) 및 `#[get("/")]` 설정
    - `mod.rs`에 `root` 모듈 선언
  * **Code:** `api/src/main.rs` 구현
    - `dotenvy::dotenv().ok();` 호출하여 `.env` 파일 로드
    - `simple_logger` 초기화
    - `api::create_rocket()` 호출하여 Rocket 인스턴스 가져오기
    - `.launch()` 메서드로 서버 실행 (`#[rocket::main]` 어트리뷰트 사용)
  * **Test:** `api/tests/root_test.rs` 재실행하여 테스트 통과 확인
  * **Docs:** 이 단계의 상세 내용과 결과 파일을 `docs/guide_01.md` 에 기록 (`Cargo.toml` 변경 사항 포함)

## 4. 테스트 전략 (`api` 패키지)
  * `api/tests/` 디렉토리 내 통합 테스트 우선 작성
  * 각 테스트 실행 전 `api/.env.test` 파일의 설정을 로드 (`dotenvy::from_filename` 또는 `tests/common` 활용)
  * `lib::create_rocket` 함수를 사용하여 일관된 테스트 환경 구성
  * 공통 테스트 로직은 `tests/common/` 모듈에 작성하여 재사용
  * 각 API 엔드포인트별 성공/실패 케이스 테스트 포함
  * 필요 시 `api/src/` 내부에 단위 테스트(`#[cfg(test)]`) 작성

# 추가 파일
1.  `./Cargo.toml` (워크스페이스 루트)
  ```toml
  [workspace]
  members = [
    "api",
  ]
  resolver = "2"

  # 워크스페이스 레벨 패키지 메타데이터 (멤버 패키지에서 상속)
  [workspace.package]
  version = "0.1.0"
  edition = "2021"
  license = "MIT"
  authors = [ "frand-nano <frand.nano@gmail.com>" ]

  # 워크스페이스 레벨 의존성 (선택 사항, 멤버 패키지에서 상속 가능)
  # [workspace.dependencies]
  # serde = { version = "1.0", features = ["derive"] }
  ```
2.  `./api/Cargo.toml` (`api` 패키지)
  ```toml
  [package]
  name = "api"
  # version, edition, license, authors 정보는 워크스페이스 루트에서 상속받음

  [lib]
  name = "api"
  path = "src/lib.rs"

  [[bin]]
  name = "api_server"
  path = "src/main.rs"

  [dependencies]
  rocket = { version = "0.5.1", features = ["json"] }
  serde = { version = "1.0", features = ["derive"] } # 또는 workspace.dependencies 에서 상속
  log = "0.4"
  simple_logger = "5.0"
  dotenvy = "0.15" # .env 파일 로드를 위해 추가

  [dev-dependencies]
  # Rocket 테스트는 dev-dependencies 가 아닌 dependencies 에 rocket 이 필요함
  ```
3.  `.gitignore`
  ```gitignore
  # cargo new 에 의해 생성됨

  /target
  **/*.rs.bk
  Cargo.lock

  # 비밀 정보 / 환경 변수
  # .env 파일들은 민감 정보를 포함할 수 있으므로 Git에 포함하지 않음
  # 단, .env.example 파일은 설정 예시를 위해 포함
  .env*
  !/.env.example
  !/.env.*.example

  # IDE 및 편집기 관련 파일
  /.idea
  /.vscode
  *.swp
  *.swo

  # 운영체제 생성 파일
  .DS_Store
  Thumbs.db
  ```
4.  `api/.env.example`
  ```dotenv
  # 개발 환경 변수 예시
  ROCKET_PORT=8000
  # ROCKET_ADDRESS="0.0.0.0"
  # DATABASE_URL="개발용_DB_URL"
  ```
5.  `api/.env.test.example`
  ```dotenv
  # 테스트 환경 변수 예시
  ROCKET_PORT=8001 # 필요시 테스트용 포트 분리
  # DATABASE_URL="테스트용_DB_URL"