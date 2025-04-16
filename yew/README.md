# Yew Frontend (`yew` 패키지)

`frand-api` 워크스페이스의 웹 애플리케이션 프론트엔드입니다. Yew 프레임워크를 기반으로 구현되었습니다.

## 주요 기술

*   Rust
*   Yew (`0.21`)
*   Trunk (빌드 도구 및 개발 서버)
*   CSS (`static/style.css`)
*   yew-router (`0.18`) (라우팅)
*   wasm-bindgen, web-sys, gloo (Web API 연동)

## 프로젝트 구조

*   `Cargo.toml`: 패키지 메타데이터 및 의존성 정의.
*   `Trunk.toml`: Trunk 빌드 및 개발 서버 설정.
*   `index.html`: Yew 앱이 렌더링될 기본 HTML 파일.
*   `static/`: 정적 에셋 (CSS, 이미지 등) 디렉토리.
    *   `style.css`: 기본 스타일시트.
*   `src/`: Rust 소스 코드.
    *   `main.rs`: 애플리케이션 진입점 (`run_app` 호출).
    *   `lib.rs`: Yew 컴포넌트, 라우팅, API 호출 로직 등 구현.

## 설정

*   **`Trunk.toml`**: Trunk 빌드 및 개발 서버 관련 설정을 정의합니다.
    *   `[build]`: 빌드 결과물 경로 (`dist`), public URL, 빌드 대상 HTML (`index.html`) 설정.
    *   `[watch]`: 파일 변경 감지 시 제외할 경로 설정.
    *   `[serve]`: 개발 서버 주소, 포트 (`8080`), 브라우저 자동 열기 설정.
    *   `[[proxy]]`: 개발 시 API 요청 (`/api/`)을 백엔드 서버 (`http://localhost:8080/api/`)로 프록시 설정.
    *   `[env]`: 빌드 시점에 프론트엔드 코드에 주입할 환경 변수 정의 (`FRONTEND_API_ENDPOINT`). 이 값은 Docker 빌드 시 `.env` 파일에서 전달받습니다.
*   **`.env` (프로젝트 루트)**: `FRONTEND_API_ENDPOINT` 변수를 정의하여 Docker 빌드 시 Yew 애플리케이션이 사용할 API 경로를 설정합니다. (예: `/api/v1`)

## 빌드

Trunk 를 사용하여 웹 애플리케이션을 빌드합니다. 결과물은 `yew/dist` 디렉토리에 생성됩니다. `static` 디렉토리의 내용도 함께 복사됩니다.

```bash
# yew 디렉토리에서 실행
trunk build --release
```
Docker 배포 시에는 `deploy/yew.Dockerfile` 내에서 빌드가 자동으로 수행됩니다.

## 개발 서버 실행

Trunk 개발 서버를 사용하여 실시간 리로드와 함께 프론트엔드를 개발할 수 있습니다. API 요청은 `Trunk.toml`의 `[[proxy]]` 설정에 따라 백엔드 서버로 전달됩니다.

```bash
# yew 디렉토리에서 실행
trunk serve --open
```
개발 서버 실행 전에 API 서버가 로컬에서 실행 중이어야 합니다 (`cd ../api && cargo run` 또는 Docker Compose 사용). 개발 서버는 `http://localhost:8080` 에서 실행됩니다.

## 테스트

(Yew 컴포넌트 및 로직에 대한 테스트 전략은 아직 정의되지 않았습니다.)

```bash
# yew 디렉토리에서 실행
# cargo test
```

## 상세 정보

Yew Frontend 및 전체 시스템의 자세한 설계 및 구현 명세는 프로젝트 루트의 [docs/spec_02.md](../../docs/spec_02.md) 파일을 참고하십시오.
