# 정보 요약
* **주요 기능:** Item CRUD (생성, 읽기, 수정, 삭제)
* **사용 API:** `GET /items`, `POST /items`, `GET /items/{id}`, `PUT /items/{id}`, `DELETE /items/{id}`
* **API 기본 경로:** `/api` (Yew 앱과 동일한 호스트에서 프록시를 통해 API를 제공한다고 가정)
* **공유 모델:** `common` 크레이트에서 `Item`, `ItemData` 구조체 정의 및 공유.
    * `Item`: `id` (String, 백엔드 ObjectId), `title` (String), `message` (String)
    * `ItemData`: `title` (String), `message` (String) - 생성/수정 시 사용
* **UI:** 간단한 단일 페이지 인터페이스 (목록 + 폼)
* **컴포넌트 구조:**
    * `App`: 메인 컴포넌트
    * `ItemList`: 아이템 목록 표시 및 상호작용
    * `ItemForm`: 아이템 생성/수정 폼
* **상태 관리:** Yew 로컬 상태 (`use_state`) 로 시작, 필요시 `Yewdux` 고려.
* **API 연동:** `gloo-net`, `wasm-bindgen-futures`, `serde`, `serde_json` 사용.
* **오류 처리:** UI에 사용자 친화적 메시지 표시, `Result` 타입 활용, 콘솔 로깅.
* **모듈 구조:** `main.rs`, `components/`, `api.rs` 분리. (`models.rs`는 `common` 크레이트로 이동)
* **필요 라이브러리:**
    * `yew`: `yew`, `gloo-net`, `wasm-bindgen-futures`, `serde`, `serde_json`, `log`, `wasm-logger`, `common` (워크스페이스 의존성)
    * `common`: `serde`

# 구현 가이드
## 1. 프로젝트 설정
  * **`common` 크레이트 생성:**
    * 워크스페이스 루트에 `common` 디렉토리 및 `Cargo.toml`, `src/lib.rs` 생성.
    * `common/Cargo.toml`에 `serde` 의존성 추가. (추가 파일 참조)
    * `common/src/lib.rs`에 `Item` 및 `ItemData` 구조체 정의. (추가 파일 참조)
  * **`yew` 크레이트 설정:**
    * `yew/Cargo.toml` 파일에 필요한 의존성 및 `common` 크레이트 의존성 추가. (추가 파일 참조)
    * 로깅 설정을 위해 `wasm-logger::init(wasm_logger::Config::default());` 와 같이 `wasm-logger`를 초기화한다. (`main.rs`의 `main` 함수 시작 부분)
  * **`api` 크레이트 설정 (별도 가이드에서 다루지만 참고용):**
    * `api/Cargo.toml`에 `common` 크레이트 의존성 추가 필요.
    * `api` 프로젝트 내 모델 정의를 `common` 크레이트 참조로 변경 필요. (ObjectId 처리 방식 주의)

## 2. 기본 구조 및 모듈화 (`yew` 크레이트 기준)
  * `main.rs`: 애플리케이션 진입점. `App` 컴포넌트를 마운트한다. (`yew::Renderer::<App>::new().render();`)
  * `api.rs`:
    * API 기본 경로를 상수로 정의한다.
      ```rust
      // 리버스 프록시 사용 가정. 직접 호출 시 "http://localhost:8080/api"
      const API_ROOT: &str = "/api";
      ```
    * `item` API 엔드포인트 호출 함수들을 구현한다. (`gloo_net::http::Request` 사용)
      * `get_items()`: `GET /api/items` 호출 (결과: `Result<Vec<Item>, gloo_net::Error>`)
      * `create_item()`: `POST /api/items` 호출 (입력: `&ItemData`, 결과: `Result<Item, gloo_net::Error>`) - `json()` 메소드로 요청 본문 설정
      * `update_item()`: `PUT /api/items/{id}` 호출 (입력: `item_id: &str`, `data: &ItemData`, 결과: `Result<Item, gloo_net::Error>`) - `json()` 메소드로 요청 본문 설정
      * `delete_item()`: `DELETE /api/items/{id}` 호출 (입력: `item_id: &str`, 결과: `Result<(), gloo_net::Error>`)
    * 각 함수는 `async fn`으로 선언하고 `wasm_bindgen_futures::spawn_local` 내에서 호출되어야 한다.
    * API 응답을 `.json::<ResponseType>()`으로 디코딩하고 `.await?`를 사용하여 결과를 얻는다.
    * 필요한 `use` 구문을 추가한다. (예: `common::{Item, ItemData}`, `gloo_net::http::Request`, `gloo_net::Error`)
  * `components/mod.rs`: 컴포넌트 모듈을 외부에 공개한다. (`pub mod item_list; pub mod item_form;`)
  * `components/item_list.rs`: `ItemList` 함수형 컴포넌트를 구현한다.
    * Props: `items: Vec<Item>`, `on_delete: Callback<String>`, `on_edit: Callback<String>` (ID 전달로 변경)
    * 아이템 목록을 순회하며 각 아이템의 `title`, `message` 및 수정/삭제 버튼을 렌더링한다.
    * 버튼 클릭 시 해당 `Callback`을 호출한다. (`item.id`가 `Some`일 때만 버튼 활성화/호출)
  * `components/item_form.rs`: `ItemForm` 함수형 컴포넌트를 구현한다.
    * Props: `item_to_edit: Option<Item>`, `on_submit: Callback<ItemData>`
    * `use_state`를 사용하여 폼 입력(`title`, `message`) 상태를 관리한다. (`ItemData` 구조체 또는 개별 필드)
    * `item_to_edit` prop이 변경될 때 `use_effect_with_deps`를 사용하여 폼 상태를 업데이트한다. (ID는 별도 관리 또는 `item_to_edit`에서 참조)
    * 폼 제출 시 현재 폼 상태로 `ItemData`를 구성하여 `on_submit` 콜백을 호출한다.
  * `App.rs` (또는 `main.rs` 내): 최상위 `App` 함수형 컴포넌트를 구현한다.

## 3. 상태 관리 및 데이터 흐름 (`App` 컴포넌트 중심)
  * `items: UseStateHandle<Vec<Item>>`: 아이템 목록 상태.
  * `editing_item_id: UseStateHandle<Option<String>>`: 현재 수정 중인 아이템의 ID 상태. (폼 데이터는 `ItemForm`이 관리하도록 변경 가능)
  * `loading: UseStateHandle<bool>`: 데이터 로딩 상태.
  * `error: UseStateHandle<Option<String>>`: 오류 메시지 상태.
  * `use_effect_with_deps` (마운트 시): `api::get_items`를 비동기로 호출하고 결과를 `items` 상태에 반영한다. 로딩 및 오류 상태를 관리한다.
  * 아이템 삭제 `Callback` 구현 (`ItemList`에서 ID 수신):
    * `api::delete_item` 호출.
    * 성공 시 `items` 상태에서 해당 아이템 제거.
    * 실패 시 `error` 상태 업데이트.
  * 아이템 수정 모드 진입 `Callback` 구현 (`ItemList`에서 ID 수신):
    * `editing_item_id` 상태를 업데이트한다.
  * 아이템 생성/수정 `Callback` 구현 (`ItemForm`에서 `ItemData` 수신):
    * `editing_item_id` 상태 유무에 따라 `api::create_item` 또는 `api::update_item` 호출. (update 시 ID 필요)
    * 성공 시 `items` 상태 업데이트 (생성: 추가, 수정: 교체) 및 `editing_item_id` 초기화 (`None`).
    * 실패 시 `error` 상태 업데이트.
  * `App` 컴포넌트의 `html!` 매크로 내에서 상태에 따라 UI 렌더링:
    * 로딩 중 표시.
    * 오류 메시지 표시.
    * `ItemForm` 렌더링 (props: `item_to_edit` (ID 기반으로 `items`에서 찾아 전달), `on_submit` 콜백 전달).
    * `ItemList` 렌더링 (props: `items`, `on_delete`, `on_edit` 콜백 전달).

## 4. UI 구현 (간단 버전)
  * `App` 컴포넌트의 `html!` 매크로에서 `ItemForm`과 `ItemList`를 순서대로 배치한다.
  * `ItemList`: HTML `<table>` 또는 `<ul>`/`<li>` 태그를 사용하여 목록 표시. 각 행/항목에 수정/삭제 버튼 포함.
  * `ItemForm`: HTML `<form>`, `<input type="text">`, `<textarea>`, `<button type="submit">` 사용.
  * `style.css`: 기본적인 레이아웃, 간격, 버튼 스타일 등을 정의한다. (`index.html`에서 로드)

## 5. 오류 처리
  * `api.rs` 함수들은 `Result<_, gloo_net::Error>`를 반환하도록 한다.
  * `App` 컴포넌트에서 `spawn_local` 내 API 호출 후 `Result`를 `match` 또는 `if let Err(e) = ...` 구문으로 처리한다.
  * 오류 발생 시 `error.set(Some(format!("오류 발생: {}", e)))` 와 같이 사용자에게 보여줄 메시지를 `error` 상태에 저장한다.
  * `ItemForm`에서 제출 전에 간단한 유효성 검사 (예: `title`이 비어 있지 않은지)를 추가할 수 있다.

## 6. 주요 참고사항
  * **ObjectId 처리:** `common` 크레이트의 `Item` 모델은 `id`를 `Option<String>`으로 다룹니다. 백엔드(`api`)에서는 이 `String`을 MongoDB의 `ObjectId`로 변환하거나, DB 전용 모델을 사용하는 등 적절한 처리가 필요합니다.
  * **API 경로:** `api.rs`의 `API_ROOT`는 리버스 프록시 사용을 가정합니다. 로컬 개발 환경에서 `trunk serve`와 Actix 서버를 별도로 실행하는 경우, `trunk`의 프록시 설정(`--proxy-backend`)을 사용하거나 `API_ROOT`를 절대 경로(`http://localhost:8080/api`)로 변경해야 할 수 있습니다.
  * **비동기 처리:** Yew에서 API 호출과 같은 비동기 작업은 반드시 `wasm_bindgen_futures::spawn_local`을 사용하여 실행해야 합니다.
  * **오류 처리 전략:** 현재 가이드는 `gloo_net::Error`를 기본으로 사용하지만, `thiserror` 등을 이용해 `yew/src/api.rs` 내에 커스텀 오류 타입(`ApiError`)을 정의하면 `App` 컴포넌트에서 더 상세한 오류 처리가 가능합니다.
  * **상태 관리:** 애플리케이션 규모가 커지면 `use_state`만으로는 상태 관리가 복잡해질 수 있습니다. 이 경우 `Yewdux`와 같은 상태 관리 라이브러리 도입을 고려하십시오.
  * **`common` 크레이트 의존성:** `api`와 `yew` 프로젝트 모두 `common` 크레이트에 의존하므로, `common` 크레이트 변경 시 양쪽 프로젝트에 미치는 영향을 고려해야 합니다.

# 추가 파일
1.  `common/Cargo.toml`
    ```toml
    [package]
    name = "common"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    # 백엔드에서 ObjectId 직접 사용 시 필요할 수 있음
    # mongodb = { version = "2.x", optional = true, features = ["bson-serde_helpers"] }

    # [features]
    # default = []
    # backend = ["dep:mongodb"] # 백엔드 전용 의존성 활성화 예시
    ```
2.  `common/src/lib.rs`
    ```rust
    use serde::{Deserialize, Serialize};

    // DB에서 조회되거나 API 응답으로 사용될 아이템 구조체
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    pub struct Item {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] // ID는 Option<String>으로 처리
        pub id: Option<String>,
        pub title: String,
        pub message: String,
        // 백엔드 전용 필드가 있다면 #[cfg(feature = "backend")] 등으로 분리 가능
    }

    // 아이템 생성 또는 수정을 위한 데이터 구조체
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    pub struct ItemData {
        pub title: String,
        pub message: String,
    }
    ```
3.  `yew/Cargo.toml`
    ```toml
    [package]
    name = "frand-yew"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    common = { path = "../common" } # 워크스페이스 내 common 크레이트 참조

    yew = { version = "0.20", features = ["csr"] }
    # yew-router = "0.17"
    gloo-net = "0.2"
    wasm-bindgen-futures = "0.4"
    serde = { version = "1.0", features = ["derive"] } # common 에서도 사용하지만, yew 에서도 필요할 수 있음
    serde_json = "1.0"
    log = "0.4"
    wasm-logger = "0.2"
    # web-sys = { version = "0.3", features = [...] }
    # thiserror = "1.0"
    # anyhow = "1.0"
    ```
4.  `yew/src/api.rs` (구현 가이드 2번 항목 참조, `use common::{Item, ItemData};` 추가)
5.  `yew/src/components/mod.rs` (변경 없음)
6.  `yew/src/components/item_list.rs` (구현 가이드 2번 항목 참조, `use common::Item;` 추가, `on_edit` 콜백 시그니처 변경)
7.  `yew/src/components/item_form.rs` (구현 가이드 2번 항목 참조, `use common::{Item, ItemData};` 추가, `NewItem` 대신 `ItemData` 사용)
8.  `yew/src/main.rs` (또는 `App.rs`, 구현 가이드 2, 3번 항목 참조, `use common::{Item, ItemData};` 추가, 상태 및 콜백 로직 수정)