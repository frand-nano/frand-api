# 정보 요약
* **프레임워크:** Rocket
* **데이터베이스:** MongoDB
* **MongoDB 연결 설정 관리:** `.env` 파일 사용. 환경 변수: `DATABASE_USER`, `DATABASE_PASS`, `DATABASE_HOST`, `DATABASE_PORT`, `DATABASE_NAME=frand_api_db`.
* **필요 의존성:** `rocket` (json 기능 포함), `mongodb`, `serde`, `dotenvy`, `tokio`, `bson` (추가 파일 `api/Cargo.toml` 참조).
* **DB 인스턴스 관리:** Rocket의 관리 상태(`rocket::State`)를 통해 `mongodb::Database` 인스턴스 공유.
* **연결 풀링:** `mongodb` crate의 기본 설정 사용.
* **데이터 모델 (`Item`):**
  - `_id`: `Option<bson::oid::ObjectId>`
  - `name`: `String`
  - `description`: `Option<String>`
  - `created_at`: `bson::DateTime`
  - `updated_at`: `bson::DateTime`
* **API 엔드포인트 (`/items`):**
  - `POST /`: 새 Item 생성
  - `GET /`: 모든 Item 목록 조회
  - `GET /<id>`: 특정 ID의 Item 조회
  - `PUT /<id>`: 특정 ID의 Item 수정
  - `DELETE /<id>`: 특정 ID의 Item 삭제
* **설정 파일 수정:** `config.rs`에서 MongoDB 연결 정보 로드 및 연결 문자열 생성.
* **서비스 모듈 추가:** `src/services/db.rs` 파일 생성하여 DB 연결 로직 구현 (`mongodb::Database` 반환).
* **테스트 케이스 추가:** Rocket 테스트 클라이언트를 사용하여 MongoDB 연결 및 CRUD 작업 확인용 테스트 추가.
* **Docker 설정:** `docker-compose.yml`에 MongoDB 서비스 추가 및 API 서비스와 연동. 사용할 네트워크 이름은 `frand-api-network` (추가 파일 참조).
* **프론트엔드 연동:** Yew 애플리케이션에서 `/items` API를 사용하도록 구현 필요.

# 구현 가이드
## 1. 의존성 추가
  * `api` 크레이트의 `Cargo.toml` 파일에 필요한 의존성을 추가하거나 확인한다.
    - `rocket`: 웹 프레임워크. `json` 기능 활성화. (버전은 프로젝트 상황에 맞게 조정)
    - `mongodb`: MongoDB 드라이버. `tokio-runtime` 기능 활성화. (버전은 프로젝트 상황에 맞게 조정)
    - `serde`: 데이터 직렬화/역직렬화. `derive` 기능 활성화.
    - `dotenvy`: `.env` 파일 로드.
    - `tokio`: 비동기 런타임.
    - `bson`: MongoDB 데이터 타입 사용. `serde_with` 기능 고려. (버전은 프로젝트 상황에 맞게 조정)
    - `futures`: `find` 결과 처리 등에 필요할 수 있음.
  * 상세 내용은 `추가 파일` 섹션의 `api/Cargo.toml` 참조.

## 2. 설정 관리
  * 프로젝트 루트에 `.env` 파일을 생성 (또는 수정)하고 MongoDB 연결 정보를 추가한다. (형식은 `추가 파일` 섹션 참조)
    - `DATABASE_NAME`은 `frand_api_db` 로 설정한다.
    - **주의:** `.env` 파일은 민감 정보를 포함하므로 `.gitignore`에 추가해야 한다.
  * `api/src/config.rs` 파일을 수정한다.
    - `dotenvy`를 사용하여 환경 변수 (`DATABASE_USER`, `DATABASE_PASS`, `DATABASE_HOST`, `DATABASE_PORT`, `DATABASE_NAME`) 를 로드한다. (오류 처리 주의)
    - 로드된 정보를 바탕으로 MongoDB 연결 문자열 (`mongodb://<USER>:<PASS>@<HOST>:<PORT>`) 을 생성하는 함수 또는 로직을 추가한다.
    - 데이터베이스 이름을 포함하는 설정 구조체를 정의한다.

## 3. 데이터베이스 서비스 구현
  * `api/src/services/mod.rs` 에 `db` 모듈을 추가한다 (`pub mod db;`).
  * `api/src/services/db.rs` 파일을 생성한다.
  * MongoDB 데이터베이스 인스턴스를 초기화하는 비동기 함수 (`init_db` 등) 를 구현한다.
    - `config.rs` 에서 생성한 연결 문자열과 데이터베이스 이름을 사용한다.
    - `mongodb::ClientOptions::parse` 와 `mongodb::Client::with_options` 를 사용하여 클라이언트를 생성한다.
    - `client.database(&db_name)` 를 호출하여 `mongodb::Database` 인스턴스를 얻는다.
    - 생성된 `mongodb::Database` 인스턴스를 반환한다 (`Result<mongodb::Database, mongodb::error::Error>`). (오류 처리 중요)

## 4. 애플리케이션 상태 공유 (Rocket)
  * `api/src/main.rs` 파일을 수정한다.
  * `#[launch]` 함수 내에서 `db::init_db()` 를 비동기적으로 호출하여 `mongodb::Database` 인스턴스를 얻는다.
    - `Result` 를 처리하여 실패 시 적절히 대응한다 (예: `expect`, 로깅 후 종료 등).
  * 얻어진 `database` 인스턴스를 Rocket의 관리 상태에 추가한다: `rocket::build().manage(database)`
  * 라우트 핸들러에서 `&State<mongodb::Database>` 타입으로 상태에 접근할 수 있다.

## 5. 데이터 모델 정의
  * `api/src/models/mod.rs` 에 `item` 모듈을 추가한다 (`pub mod item;`).
  * `api/src/models/item.rs` 파일을 생성한다.
  * `Item` 구조체를 정의한다 (확정된 필드 구조는 `정보 요약` 섹션 참조).
    - `serde::{Serialize, Deserialize}` 파생 매크로를 적용한다.
    - `#[serde(rename_all = "camelCase")]` 속성을 추가한다.
    - `_id` 필드에 `#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]` 를 적용한다.
    - `created_at`, `updated_at` 필드는 `bson::DateTime` 타입을 사용하고, 핸들러에서 적절히 값을 설정한다.
  * (권장) `PUT` 요청 처리를 위해 필드가 `Option` 타입인 `UpdateItemPayload` 구조체를 별도로 정의한다.

## 6. API 라우트 및 핸들러 구현 (Rocket)
  * `api/src/routes/mod.rs` 에 `items` 모듈을 추가한다 (`pub mod items;`).
  * `api/src/routes/items.rs` 파일을 생성한다.
  * `Item` 모델에 대한 CRUD 작업을 수행하는 Rocket 라우트 핸들러 함수들을 구현한다 (확정된 엔드포인트는 `정보 요약` 섹션 참조).
    - `#[post("/")]`, `#[get("/")]`, `#[get("/<id>")]`, `#[put("/<id>")]`, `#[delete("/<id>")]` 어트리뷰트를 사용한다.
    - 각 핸들러는 파라미터로 `&State<mongodb::Database>` 와 필요한 경우 `Json<Item>`, `Json<UpdateItemPayload>` 등을 받는다.
    - 상태에서 데이터베이스 인스턴스를 얻는다: `let db = &*state;`
    - 컬렉션 객체를 얻는다: `let collection = db.collection::<Item>("items");`
    - `collection.insert_one()`, `collection.find_one()`, `collection.find()`, `collection.update_one()`, `collection.delete_one()` 등의 메소드를 사용하여 CRUD 작업을 구현한다. `await` 키워드를 사용한다. (각 메소드의 `Result` 처리 중요)
      - `find` 사용 시 `futures::stream::TryStreamExt` 를 사용하여 결과를 `Vec`으로 수집할 수 있다.
    - 경로 파라미터 `id`는 `String`으로 받아 `bson::oid::ObjectId::parse_str(&id)` 로 변환하고 오류 처리를 수행한다. (오류 시 400 Bad Request 또는 422 Unprocessable Entity 반환 고려)
    - 생성/수정 시 `created_at`, `updated_at` 타임스탬프를 `bson::DateTime::now()` 로 설정한다.
    - 결과를 `Json` 형식 또는 `rocket::response::status` 등으로 반환한다. (성공: 200, 201, 204 / 실패: 400, 404, 500 등)
  * `main.rs` 에서 `items` 라우트를 마운트한다: `.mount("/items", routes![items::create, items::get_all, items::get_one, items::update, items::delete])` (함수 이름은 실제 구현에 맞게 조정).

## 7. 테스트 케이스 추가 (Rocket)
  * `api/tests/` 디렉토리에 `items_test.rs` 파일을 추가한다.
  * `common` 모듈 또는 테스트 파일 내에서 테스트용 Rocket 인스턴스를 설정한다.
    - 실제 `init_db`를 호출하여 테스트 데이터베이스를 사용하거나, Mock 객체를 관리 상태에 추가한다.
  * `rocket::local::blocking::Client` 를 생성하여 API 요청을 보낸다.
  * 각 CRUD 엔드포인트에 대한 통합 테스트 케이스를 작성한다.
    - Item 생성 -> `POST /items` -> 상태 코드 201 확인, 반환된 ID 확인.
    - Item 전체 조회 -> `GET /items` -> 상태 코드 200 확인, 반환된 목록 확인.
    - Item 단일 조회 -> `GET /items/<id>` -> 상태 코드 200 확인, 반환된 데이터 확인.
    - Item 수정 -> `PUT /items/<id>` -> 상태 코드 200 확인, DB 변경 확인 (별도 조회).
    - Item 삭제 -> `DELETE /items/<id>` -> 상태 코드 200 또는 204 확인, DB 제거 확인 (별도 조회).
    - 존재하지 않는 ID 조회/수정/삭제 -> 상태 코드 404 확인.
    - 잘못된 ID 형식 -> 상태 코드 400 또는 422 확인.
    - (중요) 각 테스트 후 테스트 데이터를 정리하는 로직을 포함한다.

## 8. Docker 설정
  * `deploy/docker-compose.yml` 파일을 수정하여 MongoDB 서비스를 추가한다.
    - `mongo` 서비스 정의: `image: mongo:latest`, 포트 매핑 (`27017:27017`), 볼륨 설정 (`mongo-data:/data/db`), 환경 변수 설정 (`MONGO_INITDB_ROOT_USERNAME`, `MONGO_INITDB_ROOT_PASSWORD` - `.env` 파일과 동기화 필요).
    - `api` 서비스에 `depends_on: [mongo]` 를 추가한다.
    - 최상위 레벨에 `volumes: { mongo-data: {} }` 를 추가한다.
    - `api` 서비스와 `mongo` 서비스가 통신할 수 있도록 `networks` 설정을 확인하고, 프로젝트에서 사용하는 `frand-api-network` 를 사용하도록 지정한다.
  * 상세 내용은 `추가 파일` 섹션의 `deploy/docker-compose.yml` 참조.

## 9. Yew 프론트엔드 연동 (개요)
  * `yew` 크레이트의 `Cargo.toml`에 `reqwasm`, `serde` (derive 기능) 의존성을 추가한다.
  * `yew` 크레이트 내에 `api`의 `Item` 모델과 동일한 구조의 `struct Item`을 정의한다 (`serde::{Serialize, Deserialize}` 적용).
  * API 호출을 위한 서비스 모듈 또는 함수를 구현한다.
    - `reqwasm::http::Request` 를 사용하여 `/items` 엔드포인트로 GET, POST, PUT, DELETE 요청을 보낸다.
    - 요청 본문 및 응답을 `Item` 구조체 또는 `Vec<Item>` 으로 직렬화/역직렬화한다.
    - 비동기 함수 (`async fn`) 와 `wasm_bindgen_futures::spawn_local` 을 사용하여 API 호출을 처리한다.
  * Yew 컴포넌트에서 상태 관리 로직을 구현한다.
    - 아이템 목록을 저장할 상태 (`use_state` 등) 를 정의한다.
    - 컴포넌트 마운트 시 또는 특정 이벤트 발생 시 API를 호출하여 상태를 업데이트한다.
    - 사용자 입력 폼을 만들어 새 아이템 생성 (POST), 기존 아이템 수정 (PUT) 기능을 구현한다.
    - 아이템 목록을 표시하고, 각 아이템 옆에 삭제 (DELETE) 버튼 등을 추가한다.
    - API 호출 중 로딩 상태, 오류 발생 시 사용자 피드백 등을 처리한다.

# 추가 파일
## 1. `api/Cargo.toml` (부분)
```toml
[dependencies]
rocket = { version = "0.5.0", features = ["json"] } # 버전은 프로젝트 상황에 맞게 조정
mongodb = { version = "2.8", features = ["tokio-runtime"] } # 버전은 프로젝트 상황에 맞게 조정
serde = { version = "1.0", features = ["derive"] }
dotenvy = "0.15"
tokio = { version = "1", features = ["full"] }
bson = { version = "2", features = ["serde_with"] } # 버전은 프로젝트 상황에 맞게 조정
futures = "0.3" # find 결과 처리를 위해 필요할 수 있음

# ... 기타 의존성
```

## 2. `.env.example`
```dotenv
# .env 파일 예시 (실제 값으로 채워야 함)
# 주의: 이 파일은 .gitignore 에 추가해야 합니다.
DATABASE_USER=your_username
DATABASE_PASS=your_password
DATABASE_HOST=localhost # Docker 사용 시 docker-compose.yml의 서비스 이름(예: mongo)으로 변경 필요
DATABASE_PORT=27017
DATABASE_NAME=frand_api_db

# Docker Compose 내 MongoDB 서비스 접근 예시
# DATABASE_HOST=mongo
```

## 3. `deploy/docker-compose.yml` (부분)
```yaml
version: '3.8'

services:
  # ... (기존 nginx, api 등 서비스 정의)

  api:
    # ... (기존 api 서비스 빌드 및 환경 설정)
    environment:
      # .env 파일 로드를 가정하거나, 여기에 직접 변수 설정 가능
      DATABASE_USER: ${DATABASE_USER} # .env 파일 값 참조
      DATABASE_PASS: ${DATABASE_PASS}
      DATABASE_HOST: mongo # Docker 내부 네트워크에서 mongo 서비스 이름으로 접근
      DATABASE_PORT: ${DATABASE_PORT}
      DATABASE_NAME: ${DATABASE_NAME}
      # ROCKET_ADDRESS: 0.0.0.0 # 외부 접근 허용 설정 확인
    depends_on:
      - mongo # api 서비스 시작 전에 mongo 서비스가 실행되도록 함
    networks: # api와 mongo가 같은 네트워크에 있도록 설정
      - frand-api-network # 프로젝트에서 사용하는 네트워크 이름으로 수정

  mongo:
    image: mongo:latest # 최신 MongoDB 이미지 사용
    container_name: frand_mongo # 컨테이너 이름 지정 (선택 사항)
    restart: unless-stopped
    environment:
      MONGO_INITDB_ROOT_USERNAME: ${DATABASE_USER} # 초기 루트 사용자 설정 (.env 값 사용)
      MONGO_INITDB_ROOT_PASSWORD: ${DATABASE_PASS} # 초기 루트 비밀번호 설정 (.env 값 사용)
    ports:
      - "27017:27017" # 호스트와 컨테이너 포트 매핑
    volumes:
      - mongo-data:/data/db # 데이터 영속성을 위한 볼륨 마운트
    networks:
      - frand-api-network # api와 같은 네트워크 사용

volumes:
  mongo-data: # 데이터 저장을 위한 명명된 볼륨 정의

networks: # 서비스 간 통신을 위한 네트워크 정의
  frand-api-network: # 프로젝트에서 사용하는 네트워크 이름으로 수정
    # 기존 정의된 네트워크를 사용하거나, 여기서 정의할 수 있음
    # 예시: 외부에서 생성된 네트워크 사용
    # external: true
    # 예시: 여기서 bridge 네트워크 정의
    driver: bridge

# ... (기존 다른 서비스 및 설정)
```

## 4. `yew/Cargo.toml` (부분 - 필요 시 추가)
```toml
[dependencies]
yew = { version = "0.21", features = ["csr"] } # 버전은 프로젝트 상황에 맞게 조정
reqwasm = "0.5"
serde = { version = "1.0", features = ["derive"] }
wasm-bindgen-futures = "0.4"
bson = { version = "2", features = ["serde_with"] } # Item 모델 공유 또는 재정의 시 필요

# ... 기타 의존성