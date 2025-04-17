# 정보 요약

* **라우팅:**
    * 메모 관련 신규 경로 추가: `/memos`, `/memos/new`, `/memos/:id/edit`.
    * `Route` enum 및 `switch` 함수는 별도 파일(`router.rs`)로 분리하여 관리.
* **파일 구조:**
    * 라우팅(`router.rs`), 데이터 타입(`types.rs`), 커스텀 훅(`hooks/use_memo_api.rs`)을 위한 별도 파일/디렉토리 생성.
* **메모 목록 페이지 (`/memos`):**
    * 새 메모 작성 버튼, 각 메모 항목에 수정/삭제 버튼 포함.
    * UI 형태: 카드 리스트.
* **메모 생성/수정:**
    * 별도 페이지 (`/memos/new`, `/memos/:id/edit`)에서 폼 형태로 구현.
    * 폼 필드: 제목(title), 내용(content).
    * 프론트엔드 유효성 검사: API와 동일 규칙 적용 (제목 1~140자, 내용 0~1400자). `validator` 크레이트 사용.
* **API 연동:**
    * HTTP 클라이언트: `gloo::net::http::Request`.
    * API 응답 처리: 성공/실패 메시지를 폼 근처 또는 관련 영역에 텍스트로 표시.
    * 로딩 상태: 간단한 스피너 표시.
    * **커스텀 훅 사용:** API 호출 및 관련 상태 관리 로직을 `use_memo_api` 훅으로 분리하여 사용.
* **데이터 처리:**
    * 날짜/시간: API에서 받은 타임스탬프를 `chrono` 크레이트를 사용하여 파싱하고 사용자 친화적인 형식으로 표시.
* **컴포넌트 구조:**
    * 페이지: `MemoListPage`, `MemoEditPage`.
    * 재사용 컴포넌트: `MemoListItem`, `MemoForm`.
    * 커스텀 훅: `use_memo_api`.
    * 상태 관리: 주로 `use_state` hook 사용 (커스텀 훅 내부 포함).
* **스타일링:** 기본 스타일링 적용 (특정 프레임워크나 테마 없음).
* **CORS:** 별도 설정 없이 진행, 문제 발생 시 대응.
* **의존성:** `yew/Cargo.toml`에 `chrono`, `validator` 추가 필요.

# 명세

이 문서는 `frand-api` 프로젝트의 Yew 프론트엔드에 메모 CRUD 기능을 추가하기 위한 명세입니다. 백엔드 API는 `spec_03.md`에 정의된 `/api/v1/memos` 엔드포인트를 사용합니다.

## 1. 프로젝트 구조 변경 사항 (`yew` 패키지)

`yew/src` 디렉토리 내에 컴포넌트, 페이지, 훅, 라우터, 타입 정의를 위한 하위 디렉토리 및 파일을 생성/수정합니다.

```
yew/
├── Cargo.toml        # (수정) chrono, validator 의존성 추가
├── src/
│   ├── components/     # (신규) 재사용 가능한 컴포넌트
│   │   ├── mod.rs
│   │   ├── memo_list_item.rs # (신규) 메모 목록의 개별 항목 컴포넌트
│   │   └── memo_form.rs      # (신규) 메모 생성/수정 폼 컴포넌트
│   ├── hooks/          # (신규) 커스텀 훅
│   │   ├── mod.rs
│   │   └── use_memo_api.rs # (신규) 메모 API 호출 및 상태 관리 훅
│   ├── pages/          # (신규) 라우팅 단위 페이지 컴포넌트
│   │   ├── mod.rs
│   │   ├── home.rs         # (기존 HomePage 컴포넌트 이동/수정)
│   │   ├── memo_list.rs    # (신규) 메모 목록 페이지 컴포넌트
│   │   ├── memo_edit.rs    # (신규) 메모 생성/수정 페이지 컴포넌트
│   │   └── not_found.rs    # (신규) 404 페이지 컴포넌트
│   ├── router.rs       # (신규) 라우팅 정의 및 스위칭 로직
│   ├── types.rs        # (신규) 프론트엔드 데이터 타입 정의
│   ├── lib.rs          # (수정) App 컴포넌트 수정, 모듈 import
│   └── main.rs         # (변경 없음)
├── static/
│   └── style.css     # (수정) 메모 관련 UI 스타일 추가
└── ... (기타 파일)
```

* **`hooks/`**: 메모 API 호출 및 관련 상태(데이터, 로딩, 오류) 관리를 위한 `use_memo_api` 커스텀 훅을 정의합니다.
* **`router.rs`**: `Route` enum 정의와 `switch` 함수를 포함합니다.
* **`types.rs`**: `MemoFrontend`, `MemoData` 등 프론트엔드에서 사용할 데이터 타입을 정의합니다.
* **`Cargo.toml`**: `chrono` (파싱 및 형식화 기능 포함) 및 `validator` (derive 기능 포함) 의존성을 추가해야 합니다.

## 2. 라우팅 (`yew/src/router.rs`)

메모 관련 페이지를 위한 라우트를 정의하고, 해당 라우트에 맞는 페이지 컴포넌트를 렌더링하는 `switch` 함수를 구현합니다.

| 경로                | 라우트 Enum 값        | 연결될 페이지 컴포넌트     | 설명                   |
| :------------------ | :-------------------- | :------------------------- | :--------------------- |
| `/`                 | `Route::Home`         | `pages::home::HomePage`    | 기존 홈페이지          |
| `/memos`            | `Route::MemoList`     | `pages::memo_list::MemoListPage` | 메모 목록 페이지       |
| `/memos/new`        | `Route::MemoCreate`   | `pages::memo_edit::MemoEditPage` | 새 메모 작성 페이지    |
| `/memos/:id/edit`   | `Route::MemoEdit { id }` | `pages::memo_edit::MemoEditPage` | 메모 수정 페이지       |
| `/404` (Not Found) | `Route::NotFound`     | `pages::not_found::NotFoundPage` | 404 페이지           |

* `MemoEditPage` 컴포넌트는 `memo_id: Option<String>` prop을 받아 생성/수정 모드를 구분합니다.

## 3. 데이터 모델 (`yew/src/types.rs`)

프론트엔드에서 사용할 데이터 구조체를 정의합니다.

| 구조체 이름      | 필드명      | 타입                 | 설명                                                                     | 비고 (Serde, Validate 등)           |
| :--------------- | :---------- | :------------------- | :----------------------------------------------------------------------- | :---------------------------------- |
| `MemoFrontend`   | `id`        | `Option<String>`     | 메모 ID (ObjectId 문자열)                                                | `#[serde(rename = "_id")]`          |
|                  | `title`     | `String`             | 메모 제목                                                                |                                     |
|                  | `content`   | `String`             | 메모 내용                                                                |                                     |
|                  | `updated_at`| `Option<DateTime<Utc>>` | 최종 수정일 (`chrono` 타입으로 파싱)                                 | `#[serde(with = "ts_milliseconds_option")]` (API 응답 형식에 따라 조정) |
|                  | `created_at`| `Option<DateTime<Utc>>` | 생성일 (`chrono` 타입으로 파싱)                                        | `#[serde(with = "ts_milliseconds_option")]` (API 응답 형식에 따라 조정) |
| `MemoData`       | `title`     | `String`             | 메모 제목 (폼 입력/API 요청용)                                           | `Default`, `Validate` derive        |
|                  |             |                      |                                                                          | `#[validate(length(min=1, max=140))]` |
|                  | `content`   | `String`             | 메모 내용 (폼 입력/API 요청용)                                           | `Default`, `Validate` derive        |
|                  |             |                      |                                                                          | `#[validate(length(max=1400))]`      |

* API 응답의 `_id` 필드(`ObjectId`)는 `Option<String>` 타입으로 처리합니다.
* 날짜/시간 필드는 API 응답 형식(예: 밀리초 타임스탬프)에 맞춰 `serde` helper (`ts_milliseconds_option` 등)를 사용하여 `chrono::DateTime<Utc>` 타입으로 직접 파싱합니다.
* `MemoData` 구조체는 `validator` 크레이트를 사용하여 유효성 검사 규칙을 정의합니다.

## 4. 핵심 컴포넌트 및 커스텀 훅

### 4.1. 커스텀 훅 (`hooks/use_memo_api.rs`)

| 항목         | 설명                                                                                                                                                                                                                          |
| :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 훅 이름      | `use_memo_api`                                                                                                                                                                                                                |
| 역할         | 메모 관련 API 호출(CRUD) 및 관련 상태(데이터 목록, 단일 데이터, 로딩, 오류) 관리 로직 캡슐화                                                                                                                                      |
| 입력 (Props) | 없음                                                                                                                                                                                                                          |
| 출력 (반환값)| 튜플 또는 구조체 형태:<br> - `memos: UseStateHandle<Vec<MemoFrontend>>`<br> - `memo: UseStateHandle<Option<MemoFrontend>>`<br> - `loading: UseStateHandle<bool>`<br> - `error: UseStateHandle<Option<String>>`<br> - `fetch_list: Callback<()>`<br> - `fetch_one: Callback<String>`<br> - `create: Callback<MemoData>`<br> - `update: Callback<(String, MemoData)>`<br> - `delete: Callback<String>` |
| 주요 로직    | - 내부적으로 `use_state`를 사용하여 memos, memo, loading, error 상태 관리.<br>- 각 CRUD 작업에 해당하는 비동기 함수 정의 (API 호출, 상태 업데이트, 오류 처리 포함).<br>- 각 비동기 함수를 트리거하는 `Callback` 생성 및 반환. |

### 4.2. `MemoListPage` (`pages/memo_list.rs`)

| 항목        | 설명                                                                                                                                                              |
| :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 역할        | 메모 목록 조회 및 표시 페이지                                                                                                                                     |
| 상태        | `use_memo_api` 훅에서 반환된 상태 핸들 사용 (`memos`, `loading`, `error`)                                                                                           |
| 주요 로직   | - `use_memo_api` 훅 호출.<br>- 컴포넌트 마운트 시 `use_memo_api`의 `fetch_list` 콜백 호출.<br>- 로딩 상태 관리 및 스피너 표시 (`loading` 핸들 사용).<br>- 오류 발생 시 텍스트 메시지 표시 (`error` 핸들 사용).<br>- 조회된 메모 목록(`memos` 핸들)을 `MemoListItem` 컴포넌트를 사용하여 렌더링.<br>- `MemoListItem`으로부터 삭제 요청 시 `use_memo_api`의 `delete` 콜백 호출. |
| Props       | 없음                                                                                                                                                              |
| 렌더링 요소 | 페이지 제목, 새 메모 작성 버튼 (`Link<Route::MemoCreate>`), 로딩 스피너, 오류 메시지 영역, 메모 카드 목록                                                              |

### 4.3. `MemoListItem` (`components/memo_list_item.rs`)

| 항목        | 설명                                                                                                                                                                                           |
| :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 역할        | 메모 목록의 개별 항목 표시 (카드 형태)                                                                                                                                                       |
| 상태        | 없음 (Stateless)                                                                                                                                                                             |
| 주요 로직   | - 전달받은 `memo` 데이터 표시 (제목, 내용 일부, `chrono`를 사용하여 형식화된 수정일 등).<br>- 수정 버튼 클릭 시 해당 메모의 수정 페이지로 이동 (`Link<Route::MemoEdit>`).<br>- 삭제 버튼 클릭 시 확인 창 표시 후 부모 컴포넌트의 `on_delete` 콜백 호출. |
| Props       | `memo: MemoFrontend`, `on_delete: Callback<String>`                                                                                                                                          |
| 렌더링 요소 | 카드 컨테이너 (`div.memo-card`), 제목(`h3`), 내용(`p`), 수정일(`small`), 수정 버튼(`Link`), 삭제 버튼(`button`)                                                                                 |

### 4.4. `MemoEditPage` (`pages/memo_edit.rs`)

| 항목        | 설명                                                                                                                                                                                                                                                                    |
| :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 역할        | 메모 생성 또는 수정 페이지                                                                                                                                                                                                                                              |
| 상태        | `use_memo_api` 훅에서 반환된 상태 핸들 사용 (`memo`, `loading`, `error`).                                                                                                                                                                                                 |
| 주요 로직   | - `use_memo_api` 훅 호출.<br>- 수정 모드(`memo_id`가 `Some`)일 경우, 컴포넌트 마운트 시 `use_memo_api`의 `fetch_one` 콜백 호출하여 초기 데이터 로드.<br>- 로딩 상태 관리 및 스피너 표시 (`loading` 핸들 사용).<br>- 오류 발생 시 텍스트 메시지 표시 (`error` 핸들 사용).<br>- `MemoForm` 컴포넌트에 초기 데이터(`memo` 핸들 값 기반으로 `MemoData` 생성) 및 `on_submit` 콜백 전달.<br>- `on_submit` 콜백: `MemoForm`에서 제출된 데이터로 `use_memo_api`의 `create` 또는 `update` 콜백 호출, 성공 시 성공 메시지 표시 후 목록 페이지로 이동, 실패 시 오류 메시지 표시. |
| Props       | `memo_id: Option<String>`                                                                                                                                                                                                                                               |
| 렌더링 요소 | 페이지 제목 (생성/수정 구분), 로딩 스피너, 오류/성공 메시지 영역, `MemoForm` 컴포넌트, 목록으로 돌아가기 버튼/링크                                                                                                                                                            |

### 4.5. `MemoForm` (`components/memo_form.rs`)

| 항목        | 설명                                                                                                                                                                                                                                                         |
| :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 역할        | 메모 제목과 내용을 입력받는 재사용 가능한 폼                                                                                                                                                                                                                 |
| 상태        | `form_data: UseStateHandle<MemoData>`, `validation_errors: UseStateHandle<Option<validator::ValidationErrors>>`                                                                                                                                                  |
| 주요 로직   | - 입력 필드(input, textarea) 변경 시 `form_data` 상태 업데이트.<br>- 폼 제출 시 `validator` 크레이트를 사용하여 `form_data` 유효성 검사 수행 (`form_data.validate()`).<br>- 유효성 검사 실패 시 `validation_errors` 상태 업데이트 및 필드별 오류 메시지 표시.<br>- 유효성 검사 성공 시 부모 컴포넌트의 `on_submit` 콜백 호출. |
| Props       | `initial_data: MemoData`, `on_submit: Callback<MemoData>`                                                                                                                                                                                                    |
| 렌더링 요소 | `form` 태그, 유효성 검사 오류 메시지 영역 (필드별 표시 가능), 제목 입력 필드(`input[type=text]`), 내용 입력 필드(`textarea`), 저장 버튼(`button[type=submit]`)                                                                                                     |

## 5. API 호출 및 상태 관리 (커스텀 훅 내부 로직)

* **API 호출:**
    * `use_memo_api` 훅 내부에서 `gloo::net::http::Request`를 사용하여 각 엔드포인트(GET, POST, PUT, DELETE) 호출.
    * 요청 본문은 `MemoData` 구조체를 JSON으로 직렬화하여 전송 (`.json(&data)?`).
    * 응답 본문은 `ApiResponse<T>` 래퍼를 고려하여 파싱. 성공 시 `data` 필드의 `MemoFrontend` 또는 `Vec<MemoFrontend>`으로 역직렬화 (`.json::<ApiResponse<T>>().await?`).
    * API 엔드포인트 경로는 `option_env!("FRONTEND_API_ENDPOINT")`를 사용하여 동적으로 구성.
* **상태 관리:**
    * `use_state` hook을 사용하여 훅 내부에서 `memos`, `memo`, `loading`, `error` 상태 관리.
    * 비동기 작업(API 호출)은 `wasm_bindgen_futures::spawn_local` 내에서 수행하고, 완료 시 `set` 메서드를 호출하여 상태 업데이트.
* **로딩 상태 표시:**
    * API 요청 시작 시 `loading` 상태를 `true`로 설정하고, 요청 완료(성공 또는 실패) 시 `false`로 설정.
* **오류/성공 처리:**
    * API 호출 실패(`gloo::net::Error`) 또는 응답 상태 코드가 2xx가 아닌 경우 `error` 상태에 오류 메시지(문자열) 저장.
    * API 응답 본문에 `success: false` 와 `error` 필드가 포함된 경우(`ApiError`), 해당 `error.message`를 `error` 상태에 저장.
    * API 호출 성공 시, 관련 컴포넌트(예: `MemoEditPage`)에서 성공 메시지를 텍스트 형태로 표시. 오류 발생 시 `error` 상태의 메시지를 텍스트 형태로 표시 (주로 폼 근처).

## 6. 유효성 검사

* `MemoForm` 컴포넌트에서 폼 제출 시 `validator` 크레이트를 사용하여 유효성 검사 수행 (`form_data.validate()`).
* 검사 규칙은 `MemoData` 구조체 정의에 명시된 어트리뷰트(`#[validate(...)]`)를 따름 (제목: 1~140자, 내용: 0~1400자).
* 유효성 검사 실패 시 `validation_errors` 상태(`UseStateHandle<Option<validator::ValidationErrors>>`)를 업데이트하고, 각 필드에 해당하는 오류 메시지를 사용자에게 표시.

## 7. 스타일 (`yew/static/style.css`)

* 기본 스타일링 적용. (`style.css` 파일에 필요한 클래스 추가)
* 메모 카드(`memo-card`) 스타일 정의 (테두리, 그림자, 패딩 등).
* 폼 요소(`input`, `textarea`, `button`, `label`) 스타일 개선.
* 로딩 스피너 CSS 정의 (간단한 형태).
* 오류 메시지(`.error-message`), 성공 메시지(`.success-message`) 스타일 정의.

## 8. 구현 시 주의사항

* **의존성 추가:** `yew/Cargo.toml`에 `chrono` (features: `["serde"]`), `validator` (features: `["derive"]`) 의존성을 추가해야 합니다.
* **API 응답 파싱:** API 응답의 날짜 형식이 `ts_milliseconds_option`과 호환되는지 확인하고, 필요시 `serde` 설정을 조정합니다.
* **오류 처리 상세화:** `validator::ValidationErrors`를 파싱하여 필드별 오류 메시지를 표시하는 로직 구현. 네트워크 오류, 서버 오류 등 다양한 오류 상황에 대한 사용자 피드백 구체화.
* **날짜 형식화:** `chrono`를 사용하여 `DateTime<Utc>`를 사용자 친화적인 문자열로 변환하는 로직 구현 (예: `strftime`).
* **사용자 경험:** 로딩 스피너의 위치 및 표시 방식, 성공/오류 메시지의 명확성, 삭제 확인 절차 등을 고려합니다.
