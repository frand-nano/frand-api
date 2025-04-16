# 정보 요약

* **메모(Memo) 데이터 모델 (`models/memo.rs`)**
  * 구조체명: `Memo`
  * 필드:
    * `id`: `Option<ObjectId>` (MongoDB 내부 ID, `#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]`)
    * `title`: `String` (필수, 1 ~ 140자)
    * `content`: `String` (필수, 0 ~ 1400자, 빈 문자열 허용)
    * `created_at`: `DateTime<Utc>` (자동 생성)
    * `updated_at`: `DateTime<Utc>` (자동 생성/업데이트)
  * 참고: `id`, `created_at`, `updated_at` 필드는 서버에서 관리하며, 클라이언트 요청 시에는 포함되지 않습니다.

* **API 엔드포인트 (`handlers/memo.rs`)**
  * 기본 경로: `/api/v1/memos`
  * CRUD 작업:
    * 생성: `POST /api/v1/memos`
    * 목록 조회: `GET /api/v1/memos`
    * 단일 조회: `GET /api/v1/memos/{id}`
    * 수정: `PUT /api/v1/memos/{id}` (전체 필드 업데이트)
    * 삭제: `DELETE /api/v1/memos/{id}`
  * 페이징 및 정렬: 향후 구현 (이번 명세에서는 제외)
  * ID 형식: 경로 파라미터 `{id}`는 MongoDB `ObjectId`의 16진수 문자열(String) 형식입니다.

* **요청 및 응답 형식**
  * 요청 본문 (JSON):
    * 생성 (POST): `{ "title": "...", "content": "..." }` (title, content 필수)
    * 수정 (PUT): `{ "title": "...", "content": "..." }` (title, content 필수)
  * 성공 응답 (JSON): `ApiResponse<T>` 래퍼 사용
    * 생성/단일 조회/수정: `ApiResponse<Memo>` (생성/수정된 메모 객체 반환)
    * 목록 조회: `ApiResponse<Vec<Memo>>` (메모 객체 배열 반환)
    * 삭제: `ApiResponse<()>` (data 필드가 없는 형태)
  * 입력값 유효성 검사: `validator` 크레이트 사용.

* **데이터베이스 연동 (`main.rs`, `lib.rs`, `db.rs` 등)**
  * 데이터베이스 이름: `frand_api_db` (환경 변수 `DATABASE_NAME` 사용)
  * 컬렉션 이름: `memos`
  * `mongodb` 크레이트 버전: `2.8.2` (또는 최신 안정 버전)
  * DB 핸들 관리: Rocket managed state (`State<mongodb::Database>`) 사용
    * `main.rs`에서 MongoDB 클라이언트 초기화 및 `Database` 객체 생성 후 Rocket 인스턴스에 추가.
    * 핸들러에서 `State<Database>`를 통해 DB 핸들 접근.

* **오류 처리 (`error.rs`, 핸들러 내부)**
  * 주요 오류 코드 및 상황:
    * `VALIDATION_ERROR` (400 Bad Request): 요청 본문 유효성 검사 실패 (필수 필드 누락, 길이 제한 위반 등). `details` 필드에 상세 정보 포함.
    * `NOT_FOUND` (404 Not Found): 존재하지 않는 `{id}`로 조회/수정/삭제 시도.
    * `BAD_REQUEST` (400 Bad Request): 잘못된 `{id}` 형식 (ObjectId 변환 실패).
    * `INTERNAL_SERVER_ERROR` (500 Internal Server Error): 데이터베이스 작업 실패 등 예측하지 못한 서버 오류.
  * `ApiError` 구조체를 사용하여 일관된 오류 응답 형식 유지.

* **테스트 케이스 (`tests/memo.rs`)**
  * 통합 테스트 구성:
    * 성공 케이스: 생성 -> 단일 조회 -> 목록 조회 -> 수정 -> 단일 조회 -> 삭제 -> 단일 조회 (NotFound 확인)
    * 실패 케이스: 잘못된 ID 형식, 존재하지 않는 ID 조회/수정/삭제, 유효하지 않은 입력값(길이 제한 등)으로 생성/수정 시도.
  * 테스트 환경: 별도의 테스트 데이터베이스 사용 (`frand_api_db_test`, 환경 변수 `DATABASE_NAME_TEST` 사용). 각 테스트 실행 전후 데이터 정리 로직 필요 (예: 테스트 시작 시 컬렉션 비우기).

* **기타**
  * 특별히 고려할 추가 사항 없음.

# 명세

이 문서는 `frand-api` 프로젝트에 MongoDB를 이용한 메모(Memo) CRUD API 기능을 추가하기 위한 명세입니다.

## 1. 프로젝트 구조 변경 사항

`api` 패키지 내부에 다음과 같은 파일/디렉토리가 추가되거나 수정됩니다.

```
api/
├── Cargo.toml        # (수정) mongodb, chrono, validator, futures 의존성 추가
├── src/
│   ├── config.rs     # (수정) DATABASE_NAME 환경 변수 로드 추가
│   ├── db.rs         # (신규) MongoDB 상호작용 로직 모듈 (선택 사항, 권장)
│   ├── error.rs      # (수정) 필요시 오류 처리 로직 추가 (예: Validation Error 변환)
│   ├── handlers/
│   │   ├── mod.rs    # (수정) memo 핸들러 모듈 추가
│   │   └── memo.rs   # (신규) 메모 CRUD 핸들러 구현
│   ├── lib.rs        # (수정) DB State 추가, memo 라우트 마운트
│   ├── main.rs       # (수정) MongoDB 클라이언트 초기화 및 State 추가 로직
│   └── models/
│       ├── mod.rs    # (수정) memo 모델 모듈 추가
│       └── memo.rs   # (신규) Memo 모델 및 요청 DTO 정의
└── tests/
    ├── common/
    │   └── mod.rs  # (수정) 테스트 DB 설정 및 정리 함수 추가 (선택 사항)
    └── memo.rs     # (신규) 메모 CRUD API 통합 테스트
```

## 2. 데이터베이스 모델 (`api/src/models/memo.rs`)

MongoDB `memos` 컬렉션에 저장될 문서의 구조와 API 요청/응답에 사용될 데이터 구조를 정의합니다.

### 2.1. `Memo` 구조체

MongoDB 문서 및 API 응답에 사용됩니다.

| 필드명      | 타입             | 설명                                                         | Serde/BSON 속성                                                                    |
| :---------- | :--------------- | :----------------------------------------------------------- | :--------------------------------------------------------------------------------- |
| `id`        | `Option<ObjectId>` | DB 자동 생성 ID                                              | `#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]`                |
| `title`     | `String`         | 메모 제목                                                    | `#[serde(default)]`                                                                |
| `content`   | `String`         | 메모 내용                                                    | `#[serde(default)]`                                                                |
| `created_at`| `Option<DateTime<Utc>>` | 생성 시각 (서버 자동 설정)                                   | `#[serde(with = "ts_milliseconds_option", default)]` (BSON 타임스탬프, 밀리초) |
| `updated_at`| `Option<DateTime<Utc>>` | 수정 시각 (서버 자동 설정)                                   | `#[serde(with = "ts_milliseconds_option", default)]` (BSON 타임스탬프, 밀리초) |

* `id`, `created_at`, `updated_at` 필드는 서버에서 관리합니다.

### 2.2. `CreateMemoRequest` 구조체 (요청 DTO)

메모 생성 요청(`POST /api/v1/memos`)의 본문에 사용됩니다.

| 필드명    | 타입     | 설명                                       | 유효성 검사 (`validator`)                                                   | Serde 속성       |
| :-------- | :------- | :----------------------------------------- | :-------------------------------------------------------------------------- | :--------------- |
| `title`   | `String` | 메모 제목 (필수)                           | `length(min = 1, max = 140)`                                                |                  |
| `content` | `String` | 메모 내용 (생략 시 빈 문자열로 처리)       | `length(max = 1400)`                                                        | `#[serde(default)]` |

### 2.3. `UpdateMemoRequest` 구조체 (요청 DTO)

메모 수정 요청(`PUT /api/v1/memos/{id}`)의 본문에 사용됩니다.

| 필드명    | 타입     | 설명                                       | 유효성 검사 (`validator`)                                                   | Serde 속성       |
| :-------- | :------- | :----------------------------------------- | :-------------------------------------------------------------------------- | :--------------- |
| `title`   | `String` | 메모 제목 (필수)                           | `length(min = 1, max = 140)`                                                |                  |
| `content` | `String` | 메모 내용 (생략 시 빈 문자열로 처리)       | `length(max = 1400)`                                                        | `#[serde(default)]` |

### 2.4. `models/mod.rs` 수정

`pub mod memo;` 를 추가하여 `memo` 모듈을 외부에 노출시킵니다.

## 3. API 엔드포인트 명세

`/api/v1/memos` 경로 아래에 다음과 같은 엔드포인트를 추가합니다.

| 메소드 | 경로                  | 설명         | 요청 본문 (Content-Type: application/json) | 성공 응답 (200 OK)                 | 주요 오류 응답                                                                 |
| :----- | :-------------------- | :----------- | :----------------------------------------- | :--------------------------------- | :----------------------------------------------------------------------------- |
| POST   | `/api/v1/memos`       | 새 메모 생성 | `CreateMemoRequest`                        | `Json<ApiResponse<Memo>>`          | 400 (VALIDATION_ERROR), 500 (INTERNAL_SERVER_ERROR)                            |
| GET    | `/api/v1/memos`       | 메모 목록 조회 | 없음                                       | `Json<ApiResponse<Vec<Memo>>>`     | 500 (INTERNAL_SERVER_ERROR)                                                    |
| GET    | `/api/v1/memos/{id}`  | 특정 메모 조회 | 없음                                       | `Json<ApiResponse<Memo>>`          | 400 (BAD_REQUEST - Invalid ID), 404 (NOT_FOUND), 500 (INTERNAL_SERVER_ERROR) |
| PUT    | `/api/v1/memos/{id}`  | 특정 메모 수정 | `UpdateMemoRequest`                        | `Json<ApiResponse<Memo>>`          | 400 (BAD_REQUEST/VALIDATION_ERROR), 404 (NOT_FOUND), 500 (INTERNAL_SERVER_ERROR) |
| DELETE | `/api/v1/memos/{id}`  | 특정 메모 삭제 | 없음                                       | `Json<ApiResponse<()>>` (No data) | 400 (BAD_REQUEST - Invalid ID), 404 (NOT_FOUND), 500 (INTERNAL_SERVER_ERROR) |

* `{id}` 경로 파라미터는 MongoDB `ObjectId`의 16진수 문자열입니다.
* 모든 성공 응답은 `response::ApiResponse<T>` 래퍼를 사용합니다.
* 모든 실패 응답은 `error::ApiError` 형식을 따릅니다.

## 4. 핵심 함수 시그니처 및 로직

### 4.1. 데이터베이스 상호작용 (`api/src/db.rs` - 권장)

데이터베이스 관련 로직을 별도 모듈로 분리합니다.

| 함수명            | 주요 역할                      | 입력 파라미터                             | 반환 타입                        | 비고                                              |
| :---------------- | :----------------------------- | :---------------------------------------- | :------------------------------- | :------------------------------------------------ |
| `get_memo_collection` | `memos` 컬렉션 핸들 반환       | `db: &Database`                         | `Collection<Memo>`               |                                                   |
| `create_memo_db`  | 새 메모 문서 생성 및 DB 삽입   | `db: &Database`, `req: CreateMemoRequest` | `Result<Memo, mongodb::Error>`   | `created_at`, `updated_at` 설정, 삽입된 문서 반환 |
| `list_memos_db`   | 모든 메모 문서 조회            | `db: &Database`                         | `Result<Vec<Memo>, mongodb::Error>` | `TryStreamExt` 사용 필요                          |
| `get_memo_db`     | ID로 특정 메모 문서 조회       | `db: &Database`, `id: ObjectId`         | `Result<Option<Memo>, mongodb::Error>` |                                                   |
| `update_memo_db`  | ID로 특정 메모 문서 수정       | `db: &Database`, `id: ObjectId`, `req: UpdateMemoRequest` | `Result<Option<Memo>, mongodb::Error>` | `updated_at` 갱신, 수정 후 문서 반환(`ReturnDocument::After`) |
| `delete_memo_db`  | ID로 특정 메모 문서 삭제       | `db: &Database`, `id: ObjectId`         | `Result<Option<DeleteResult>, mongodb::Error>` | 삭제 결과 반환                              |

### 4.2. 요청 핸들러 (`api/src/handlers/memo.rs`)

각 API 엔드포인트에 대한 요청을 처리합니다. `State<Database>`를 통해 DB 핸들에 접근합니다.

| 핸들러 함수명        | HTTP 메소드 | 경로                | 주요 로직                                                                                                                              | 반환 타입                             |
| :------------------- | :---------- | :------------------ | :------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------ |
| `create_memo_handler`| POST        | `/`                 | 입력값 유효성 검사(`Validate`), `db::create_memo_db` 호출, 성공 시 `ApiResponse<Memo>` 반환.                                           | `Result<Json<ApiResponse<Memo>>, ApiError>` |
| `list_memos_handler` | GET         | `/`                 | `db::list_memos_db` 호출, 성공 시 `ApiResponse<Vec<Memo>>` 반환.                                                                       | `Result<Json<ApiResponse<Vec<Memo>>>, ApiError>` |
| `get_memo_handler`   | GET         | `/<id>`             | `id` 파싱 (`ObjectId`), `db::get_memo_db` 호출, 결과 없으면 404(NOT_FOUND), 성공 시 `ApiResponse<Memo>` 반환.                             | `Result<Json<ApiResponse<Memo>>, ApiError>` |
| `update_memo_handler`| PUT         | `/<id>`, data=`<req>` | `id` 파싱, 입력값 유효성 검사, `db::update_memo_db` 호출, 결과 없으면 404(NOT_FOUND), 성공 시 `ApiResponse<Memo>` 반환.               | `Result<Json<ApiResponse<Memo>>, ApiError>` |
| `delete_memo_handler`| DELETE      | `/<id>`             | `id` 파싱, `db::delete_memo_db` 호출, 삭제된 문서 없으면 404(NOT_FOUND), 성공 시 `ApiResponse<()>` (No data) 반환.                 | `Result<Json<ApiResponse<()>>, ApiError>`   |

* **`handlers/mod.rs`**: `pub mod memo;` 를 추가합니다.
* **`lib.rs` (`build_rocket` 함수 내부)**: `memo` 핸들러들을 `/api/v1/memos` 경로에 마운트합니다.

### 4.3. MongoDB 클라이언트 초기화 (`api/src/main.rs`)

애플리케이션 시작 시 MongoDB 클라이언트를 초기화하고 Rocket 상태로 관리합니다.

* `main` 함수의 반환 타입을 `Result<(), Box<dyn std::error::Error>>` 등으로 변경하여 MongoDB 연결 오류 처리.
* `Config`에서 `mongodb_uri()` 와 `database_name` 을 가져옵니다.
* `mongodb::options::ClientOptions::parse()` 와 `mongodb::Client::with_options()` 를 사용하여 비동기적으로 클라이언트 생성.
* `client.database()` 를 사용하여 `Database` 핸들 획득.
* `api::build_rocket(config).manage(db)` 를 호출하여 `Database` 핸들을 Rocket 의 상태(State)로 등록.

### 4.4. 설정 로드 수정 (`api/src/config.rs`)

`DATABASE_NAME` 환경 변수를 로드하도록 수정합니다.

* `Config` 구조체에 `database_name: String` 필드 추가.
* `Config::load()` 함수 내부에 `env::var("DATABASE_NAME").unwrap_or_else(|_| "frand_api_db".to_string())` 로직 추가하여 환경 변수 로드 (기본값 "frand_api_db").

## 5. 설정 파일 변경 사항

### 5.1. `api/Cargo.toml`

필요한 크레이트를 의존성에 추가합니다.

```toml
# filepath: api/Cargo.toml
[dependencies]
# ... 기존 의존성 ...
mongodb = { version = "2.8.2", default-features = false, features = ["tokio-runtime"] } # 버전 확인 및 tokio-runtime 피처 활성화
chrono = { version = "0.4", features = ["serde"] } # 시간 처리 및 serde 지원
validator = { version = "0.18", features = ["derive"] } # 유효성 검사
futures = "0.3" # async stream 처리를 위해 추가 (list_memos_db 등)
```

### 5.2. `.env` / `.env.example`

데이터베이스 이름 환경 변수를 추가합니다.

```dotenv
# filepath: .env / .env.example
# ... 기존 변수 ...
DATABASE_NAME=frand_api_db
```

### 5.3. `.env.test`

테스트용 데이터베이스 이름 환경 변수를 추가합니다.

```dotenv
# filepath: .env.test
# ... 기존 변수 ...
DATABASE_NAME=frand_api_db_test # 테스트용 DB 이름
```

## 6. 파일별 역할 및 의존성 (신규/주요 변경)

* **`models/memo.rs`**: `Memo` 데이터 모델, `CreateMemoRequest`, `UpdateMemoRequest` DTO 정의. (의존성: `serde`, `mongodb::bson`, `chrono`, `validator`)
* **`handlers/memo.rs`**: 메모 CRUD API 엔드포인트 핸들러 구현. (의존성: `rocket`, `mongodb`, `validator`, `crate::models::memo`, `crate::response`, `crate::error`, `crate::db` (선택))
* **`db.rs` (선택)**: MongoDB 컬렉션 접근 및 CRUD 로직 구현. (의존성: `mongodb`, `futures`, `crate::models::memo`)
* **`tests/memo.rs`**: 메모 CRUD API 통합 테스트 구현. (의존성: `rocket`, `serde_json`, `crate::common`, `api`)
* **`main.rs`**: MongoDB 클라이언트 초기화 및 Rocket 상태 관리 추가. (의존성: `mongodb`)
* **`config.rs`**: `DATABASE_NAME` 환경 변수 로드 로직 추가.

## 7. 오류 처리 상세

* **입력 유효성 검사 (`VALIDATION_ERROR`)**:
    * 핸들러에서 `req.validate()` 호출.
    * `ValidationErrors` 발생 시 `ApiError::new(VALIDATION_ERROR, ...).with_details(serde_json::to_value(errors)?)` 형태로 변환하여 400 응답. `details` 필드에 필드별 오류 메시지 포함.
* **ID 형식 오류 (`BAD_REQUEST`)**:
    * 핸들러에서 경로 파라미터 `id`를 `ObjectId::parse_str()`로 변환 시 발생하는 오류 처리.
    * `ApiError::new(BAD_REQUEST, "Invalid ID format", Status::BadRequest)` 반환.
* **리소스를 찾을 수 없음 (`NOT_FOUND`)**:
    * `get_memo_db`, `update_memo_db`, `delete_memo_db` 결과가 `None` 또는 `DeleteResult.deleted_count == 0` 인 경우.
    * `ApiError::new(NOT_FOUND, "Memo not found", Status::NotFound)` 반환.
* **데이터베이스 오류 (`INTERNAL_SERVER_ERROR`)**:
    * `db.rs` 또는 핸들러 내 DB 작업 중 발생하는 `mongodb::error::Error` 처리.
    * 오류 로깅 후 `ApiError::new(INTERNAL_SERVER_ERROR, "Database operation failed", Status::InternalServerError)` 반환. (민감한 오류 정보는 로깅만 하고 클라이언트에게는 일반적인 메시지 전달)

## 8. 테스트 케이스 (`api/tests/memo.rs`)

* **테스트 설정 (`common::setup_db`)**:
    * 테스트 시작 전 호출되어 테스트 DB(`frand_api_db_test`)에 연결하고 `memos` 컬렉션을 비우는 함수 구현 권장.
* **테스트 함수 구조**:
    * 각 테스트 함수는 `#[rocket::async_test]` 어트리뷰트 사용.
    * `common::setup()` 및 `common::setup_db()` 호출.
    * Rocket 테스트 클라이언트 생성 (`Client::tracked`).
    * API 요청 생성 및 전송 (`client.post("/api/v1/memos").json(...)`).
    * 응답 상태 코드, Content-Type, 본문 내용 검증.
* **주요 테스트 시나리오**:
    * `test_create_memo_success`: 유효한 데이터로 메모 생성 성공 검증.
    * `test_create_memo_fail_validation`: 유효하지 않은 데이터(길이 제한 위반 등)로 생성 시 400(VALIDATION_ERROR) 응답 검증.
    * `test_get_memo_success`: 생성된 메모 ID로 조회 성공 검증.
    * `test_get_memo_fail_invalid_id`: 잘못된 형식의 ID로 조회 시 400(BAD_REQUEST) 응답 검증.
    * `test_get_memo_fail_not_found`: 존재하지 않는 ID로 조회 시 404(NOT_FOUND) 응답 검증.
    * `test_list_memos_success`: 여러 메모 생성 후 목록 조회 성공 및 개수/내용 검증.
    * `test_update_memo_success`: 생성된 메모 수정 성공 및 내용 변경 확인.
    * `test_update_memo_fail_validation`: 유효하지 않은 데이터로 수정 시 400(VALIDATION_ERROR) 응답 검증.
    * `test_update_memo_fail_not_found`: 존재하지 않는 ID로 수정 시 404(NOT_FOUND) 응답 검증.
    * `test_delete_memo_success`: 생성된 메모 삭제 성공 (200 OK, No Data) 및 이후 조회 시 404(NOT_FOUND) 확인.
    * `test_delete_memo_fail_not_found`: 존재하지 않는 ID로 삭제 시 404(NOT_FOUND) 응답 검증.

## 9. 구현 시 주의사항

* **ObjectId 변환**: API 경로의 `id` 문자열과 MongoDB의 `ObjectId` 타입 간 변환 및 오류 처리에 유의합니다.
* **날짜/시간 처리**: `created_at`, `updated_at` 필드는 DB 작업 시 서버 시간(`Utc::now()`) 기준으로 설정해야 합니다. `chrono`와 `mongodb::bson::DateTime` 사용법을 숙지합니다.
* **비동기 처리**: 모든 DB 작업은 비동기(`async`/`await`)로 처리해야 합니다. `futures::stream::TryStreamExt` 등을 활용하여 비동기 스트림(예: 목록 조회 결과)을 처리합니다.
* **오류 매핑**: `mongodb::error::Error` 및 `validator::ValidationErrors`를 `ApiError`로 적절히 변환하고, 사용자에게 노출할 정보와 로깅할 정보를 구분합니다.
* **트랜잭션**: 현재 명세는 개별 CRUD 작업만 다루지만, 여러 DB 작업을 원자적으로 처리해야 할 경우 MongoDB 트랜잭션 사용을 고려해야 합니다. (이번 명세 범위 외)
* **상태 관리**: Rocket의 `State`를 통해 `Database` 핸들을 안전하게 공유하고 핸들러에서 사용합니다.