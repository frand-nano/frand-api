# 정보 요약
* **프레임워크:** Rocket
* **데이터베이스:** MongoDB
* **MongoDB 연결 설정 관리:** `.env` 파일 사용. 환경 변수: `DATABASE_USER`, `DATABASE_PASS`, `DATABASE_HOST`, `DATABASE_PORT`, `DATABASE_NAME=frand_api_db`.
* **필요 의존성:** `rocket` (json 기능 포함), `mongodb`, `serde`, `dotenvy`, `tokio`, `bson` (추가 파일 `api/Cargo.toml` 참조).
* **DB 인스턴스 관리:** Rocket의 관리 상태(`rocket::State`)를 통해 `mongodb::Database` 인스턴스 공유.
* **연결 풀링:** `mongodb` crate의 기본 설정 사용.
* **데이터 모델 (`Item`):**
  - `_id`: `Option<bson::oid::ObjectId>`
  - `title`: `String`
  - `message`: `String`
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
  * `api/src/models/mod.rs`에 `DBItem`, `Item`, `ItemData` 구조체를 직접 정의한다.
    - `serde::{Serialize, Deserialize}` 파생 매크로를 적용한다.
    - `DBItem`과 `Item` 구조체에 `#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]` 를 적용한다.
    - `Item::from` 구현으로 `DBItem`을 `Item`으로 변환하는 기능 추가한다.

## 6. API 라우트 및 핸들러 구현 (Rocket)
  * `api/src/routes/mod.rs` 에 `items` 모듈을 추가한다 (`pub mod items;`).
  * `api/src/routes/items.rs` 파일을 생성한다.
  * `Item` 모델에 대한 CRUD 작업을 수행하는 Rocket 라우트 핸들러 함수들을 구현한다 (확정된 엔드포인트는 `정보 요약` 섹션 참조).
    - `#[post("/")]`, `#[get("/")]`, `#[get("/<id>")]`, `#[put("/<id>")]`, `#[delete("/<id>")]` 어트리뷰트를 사용한다.
    - 각 핸들러는 파라미터로 `&State<mongodb::Database>` 와 필요한 경우 `Json<ItemData>` 등을 받는다.
    - 상태에서 데이터베이스 인스턴스를 얻는다: `let db = &*state;`
    - 컬렉션 객체를 얻는다: `let collection = db.collection::<DBItem>("items");`
    - `collection.insert_one()`, `collection.find_one()`, `collection.find()`, `collection.update_one()`, `collection.delete_one()` 등의 메소드를 사용하여 CRUD 작업을 구현한다. `await` 키워드를 사용한다. (각 메소드의 `Result` 처리 중요)
      - `find` 사용 시 `futures::stream::TryStreamExt` 를 사용하여 결과를 `Vec`으로 수집할 수 있다.
    - 경로 파라미터 `id`는 `String`으로 받아 `bson::oid::ObjectId::parse_str(&id)` 로 변환하고 오류 처리를 수행한다. (오류 시 400 Bad Request 반환 고려)
    - 결과를 `Json` 형식 또는 `rocket::response::status` 등으로 반환한다. (성공: 200, 201, 204 / 실패: 400, 404, 500 등)
  * `main.rs` 에서 `items` 라우트를 마운트한다: `.mount("/items", routes![items::create_item, items::get_all_items, items::get_item_by_id, items::update_item, items::delete_item])` (함수 이름은 실제 구현에 맞게 조정).

## 7. 테스트 케이스 추가 (Rocket)
  * `api/tests/` 디렉토리에 `items_test.rs` 파일을 추가한다.
  * `test_common` 모듈 또는 테스트 파일 내에서 테스트용 Rocket 인스턴스를 설정한다.
    - 실제 `init_db`를 호출하여 테스트 데이터베이스를 사용하도록 한다.
  * `rocket::local::asynchronous::Client` 를 생성하여 API 요청을 보낸다.
  * 각 CRUD 엔드포인트에 대한 통합 테스트 케이스를 작성한다.
    - Item 생성 -> `POST /items` -> 상태 코드 201 확인, 반환된 ID 확인.
    - Item 전체 조회 -> `GET /items` -> 상태 코드 200 확인, 반환된 목록 확인.
    - Item 단일 조회 -> `GET /items/<id>` -> 상태 코드 200 확인, 반환된 데이터 확인.
    - Item 수정 -> `PUT /items/<id>` -> 상태 코드 200 확인, DB 변경 확인 (별도 조회).
    - Item 삭제 -> `DELETE /items/<id>` -> 상태 코드 204 확인, DB 제거 확인 (별도 조회).
    - 존재하지 않는 ID 조회/수정/삭제 -> 상태 코드 404 확인.
    - 잘못된 ID 형식 -> 상태 코드 400 확인.
    - (중요) 각 테스트 전에 테스트 데이터베이스를 정리하는 로직을 포함한다.

## 8. Docker 설정
  * `deploy/docker-compose.yml` 파일을 수정하여 MongoDB 서비스를 추가한다.
    - `mongo` 서비스 정의: `image: mongo:latest`, 포트 매핑 (`27017:27017`), 볼륨 설정 (`mongo-data:/data/db`), 환경 변수 설정 (`MONGO_INITDB_ROOT_USERNAME`, `MONGO_INITDB_ROOT_PASSWORD` - `.env` 파일과 동기화 필요).
    - `api` 서비스에 `depends_on: [mongo]` 를 추가한다.
    - 최상위 레벨에 `volumes: { mongo-data: {} }` 를 추가한다.
    - `api` 서비스와 `mongo` 서비스가 통신할 수 있도록 `networks` 설정을 확인하고, 프로젝트에서 사용하는 `frand-api-network` 를 사용하도록 지정한다.
  * 상세 내용은 `추가 파일` 섹션의 `deploy/docker-compose.yml` 참조.

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
services:
  # ... (기존 nginx, api 등 서비스 정의)

  api:
    # ... (기존 api 서비스 빌드 및 환경 설정)
    environment:
      DATABASE_HOST: mongo # Docker 내부 네트워크에서 mongo 서비스 이름으로 접근
    networks:
      - frand-api-network
    restart: unless-stopped
    depends_on:
      - mongo # api 서비스 시작 전에 mongo 서비스가 실행되도록 함

  mongo:
    image: mongo:latest
    restart: unless-stopped
    environment:
      MONGO_INITDB_ROOT_USERNAME: ${DATABASE_USER}
      MONGO_INITDB_ROOT_PASSWORD: ${DATABASE_PASS}
    ports:
      - "27017:27017"
    volumes:
      - mongo-data:/data/db
    networks:
      - frand-api-network

networks:
  frand-api-network:
    driver: bridge

volumes:
  mongo-data: # 데이터 저장을 위한 명명된 볼륨 정의
```