# 정보 요약
* **주요 기능:** Item CRUD (생성, 읽기, 수정, 삭제)
* **사용 API:** `GET /items`, `POST /items`, `GET /items/{id}`, `PUT /items/{id}`, `DELETE /items/{id}`
* **API 기본 경로:** `/api` (Yew 앱과 동일한 호스트에서 프록시를 통해 API를 제공한다고 가정)
* **데이터 모델:**
    * `Item`: `id` (String, 백엔드 ObjectId), `title` (String), `message` (String)
    * `ItemData`: `title` (String), `message` (String) - 생성/수정 시 사용
    * `DBItem`: 백엔드 전용, `id` (ObjectId), `title` (String), `message` (String)
* **UI:** 간단한 단일 페이지 인터페이스 (목록 + 폼)
* **컴포넌트 구조:**
    * `App`: 메인 컴포넌트
    * `ItemList`: 아이템 목록 표시 및 상호작용
    * `ItemForm`: 아이템 생성/수정 폼
* **상태 관리:** Yew 로컬 상태 (`use_state`)
* **API 연동:** `gloo-net`, `wasm-bindgen-futures`, `serde`, `serde_json` 사용
* **오류 처리:** UI에 사용자 친화적 메시지 표시, `Result` 타입 활용, 콘솔 로깅
* **모듈 구조:** `main.rs`, `components/`, `api.rs`, `models/mod.rs`
* **필요 라이브러리:**
    * `yew`, `gloo-net`, `wasm-bindgen-futures`, `serde`, `serde_json`, `log`, `wasm-logger`

# 구현 가이드
## 1. 프로젝트 설정
  * **`yew` 크레이트 설정:**
    * `yew/Cargo.toml` 파일에 필요한 의존성 추가
    * 로깅 설정을 위해 `wasm-logger::init(wasm_logger::Config::default());` 와 같이 `wasm-logger`를 초기화
  * **정적 파일 구조:**
    * `static/` 디렉토리에 CSS, 이미지 등의 정적 파일 배치
    * `index.html`에서 스타일시트 로드 경로 변경: `/static/style.css`

## 2. 기본 구조 및 모듈화
  * `main.rs`: 애플리케이션 진입점. `App` 컴포넌트를 마운트 (`yew::Renderer::<App>::new().render();`)
  * `models/mod.rs`: 
    * `Item`: 프론트엔드와 API 응답에서 사용하는 모델 (`id` 필드는
    `Option<String>`)
    * `ItemData`: 아이템 생성 또는 수정을 위한 데이터 구조체
  * `api.rs`:
    * API 기본 경로를 상수로 정의
      ```rust
      // 리버스 프록시 사용 가정
      const API_ROOT: &str = "/api";
      ```
    * `item` API 엔드포인트 호출 함수들 구현 (`gloo_net::http::Request` 사용)
      * `get_items()`: `GET /api/items` 호출 (결과: `Result<Vec<Item>, gloo_net::Error>`)
      * `create_item()`: `POST /api/items` 호출 (입력: `&ItemData`, 결과: `Result<Item, gloo_net::Error>`)
      * `update_item()`: `PUT /api/items/{id}` 호출 (입력: `item_id: &str`, `data: &ItemData`, 결과: `Result<Item, gloo_net::Error>`)
      * `delete_item()`: `DELETE /api/items/{id}` 호출 (입력: `item_id: &str`, 결과: `Result<(), gloo_net::Error>`)
    * 각 함수는 `async fn`으로 선언하고 `wasm_bindgen_futures::spawn_local` 내에서 호출
  * `components/mod.rs`: 컴포넌트 모듈 외부에 공개 (`pub mod item_list; pub mod item_form;`)
  * `components/item_list.rs`: `ItemList` 함수형 컴포넌트 구현
    * Props: `items: Vec<Item>`, `on_delete: Callback<String>`, `on_edit: Callback<String>` (ID 전달)
    * 아이템 목록을 테이블 형태로 렌더링하고 각 행에 수정/삭제 버튼 표시
  * `components/item_form.rs`: `ItemForm` 함수형 컴포넌트 구현
    * Props: `item_to_edit: Option<Item>`, `on_submit: Callback<ItemData>`
    * `use_state`와 `use_effect_with`를 사용하여 폼 입력 상태 관리
    * 폼 제출 시 유효성 검사 후 `on_submit` 콜백 호출

## 3. 상태 관리 및 데이터 흐름 (`App` 컴포넌트)
  * `items: UseStateHandle<Vec<Item>>`: 아이템 목록 상태
  * `editing_item_id: UseStateHandle<Option<String>>`: 현재 수정 중인 아이템 ID 상태
  * `loading: UseStateHandle<bool>`: 데이터 로딩 상태
  * `error: UseStateHandle<Option<String>>`: 오류 메시지 상태
  * `use_effect_with` (마운트 시): `api::get_items`를 비동기로 호출하고 결과를 `items` 상태에 반영
  * 아이템 삭제 `Callback`: `api::delete_item` 호출 후 성공 시 목록에서 항목 제거
  * 아이템 수정 모드 진입 `Callback`: `editing_item_id` 상태 업데이트
  * 아이템 생성/수정 `Callback`: `editing_item_id` 상태에 따라 `api::create_item` 또는 `api::update_item` 호출
  * 오류 발생 시 `error` 상태 업데이트 및 콘솔 로깅

## 4. UI 구현
  * `App` 컴포넌트: 오류 메시지, 로딩 표시, `ItemForm`, `ItemList` 렌더링
  * `ItemList`: HTML `<table>` 태그를 사용하여 목록 표시, 각 행에 수정/삭제 버튼
  * `ItemForm`: HTML `<form>`, `<input>`, `<textarea>`, `<button>` 사용
  * `static/style.css`: 레이아웃, 간격, 버튼 스타일, 테이블 스타일 정의

## 5. 오류 처리
  * API 함수는 `Result<_, gloo_net::Error>`를 반환
  * `App` 컴포넌트에서 API 호출 결과를 `match` 또는 조건문으로 처리
  * 오류 발생 시 사용자에게 보여줄 메시지를 `error` 상태에 저장하고 UI에 표시
  * 콘솔 로깅을 통해 개발자가 오류 확인 가능 (`web-sys::console::error_1`)

## 6. 주요 참고사항
  * **Nginx 설정:** 정적 파일은 `/static/` 경로에서 제공되며, Nginx 설정에서 캐시 설정 추가
  * **API 경로:** 리버스 프록시를 통해 백엔드 API를 `/api` 경로로 제공
  * **비동기 처리:** API 호출과 같은 비동기 작업은 `wasm_bindgen_futures::spawn_local`을 사용하여 실행
  * **네트워크 오류 처리:** API 호출 실패 시 사용자에게 친화적인 오류 메시지 표시
  * **Docker 배포:** 프론트엔드는 빌드 후 Nginx 컨테이너에 정적 파일로 포함되어 배포

# 추가 파일
1.  `yew/Cargo.toml`
  ```toml
  [package]
  version = "0.1.2"
  edition = "2021"
  license = "MIT"
  authors = [ "frand-nano <frand.nano@gmail.com>" ]
  name = "yew"

  [dependencies]
  yew = { version = "0.21", features = ["csr"] }
  yew-router = "0.18"
  wasm-bindgen = "0.2"
  web-sys = { version = "0.3", features = ["Window", "Document", "Element", "HtmlElement", "Node", "Location"] }
  gloo-net = "0.2"
  wasm-bindgen-futures = "0.4"
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  log = "0.4"
  wasm-logger = "0.2"
  ```