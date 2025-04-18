# 기술 명세
## 프로젝트 정보
  * 라이선스: MIT
  * 초기 버전: 0.1.0
  * 개발자: frand-nano <frand.nano@gmail.com>

## 프로젝트 구조
  * 전체 프로젝트 루트 구조:
    ```
    /
    ├── api/          # Rust Rocket 백엔드
    │   ├── Cargo.toml
    │   └── src/      # API 소스 코드
    │       ├── main.rs
    │       ├── lib.rs
    │       ├── config.rs
    │       ├── error.rs
    │       └── routes/
    │           └── health.rs
    ├── yew/          # Rust Yew 프론트엔드 (향후 추가 예정)
    │   ├── Cargo.toml
    │   ├── Trunk.toml
    │   ├── index.html
    │   └── src/
    │       └── main.rs
    ├── docs/         # 프로젝트 문서화
    │   ├── spec.md   # 기술 명세
    │   └── guide.md  # 구현 가이드
    ├── config/       # API 서버 설정 파일 (default.toml, test.toml 등)
    ├── deploy/       # 배포 관련 파일 (docker-compose.yml, nginx 설정, 인증서 등)
    │   ├── .env.example # 환경 변수 예시 파일
    │   ├── api.dockerfile
    │   ├── nginx.dockerfile # Nginx Dockerfile
    │   ├── docker-compose.yml
    │   ├── config/   # API 설정 파일 (빌드 시 api.Dockerfile에서 사용)
    │   │   └── default.toml
    │   ├── nginx/    # Nginx 관련 설정 및 스크립트
    │   │   ├── nginx.conf.template # Nginx 설정 템플릿
    │   │   └── template_replace.sh # Nginx 설정 적용 및 실행 스크립트
    │   ├── certs/    # TLS 인증서 파일 위치 (cert.pem, privkey.pem) 및 생성 스크립트
    │   │   └── gen_certs.sh # TLS 인증서 생성 스크립트
    │   └── static/   # Nginx에서 서빙할 정적 파일 위치 (볼륨 마운트 대상)
    ├── Cargo.toml    # 워크스페이스 설정
    ├── .gitignore
    └── .dockerignore # Docker 빌드 시 제외할 파일 목록
    ```
  * `api/src` 내부에 `config`, `routes`, `error`, `handlers`, `models`, `services` 등의 모듈 구조 사용
  * `deploy/` 폴더 내에는 빌드된 이미지와 배포 관련 파일(docker-compose.yml, nginx 설정 등)을 위치시켜 해당 폴더만으로 배포 가능하도록 구성.
  * `yew/` 폴더는 향후 Yew 프론트엔드 관련 파일을 위치시키고, Trunk 빌드 도구를 사용하여 WASM 앱을 빌드하도록 구성 예정.

## 의존성
  * **Backend (API)**
    * 프로그래밍 언어: Rust
    * 웹 프레임워크: Rocket (JSON 기능 활성화)
    * 로깅: `log`, `simple_logger` 크레이트 사용 (터미널 출력)
    * 오류 처리: `anyhow`, `thiserror` 사용.
      - (추천 방식) `thiserror`로 사용자 정의 오류 타입을 정의하고, Rocket의 `Responder`를 구현하여 오류를 적절한 HTTP 응답으로 변환하는 방식 고려.
    * 설정 관리:
        - API 서버 내부 설정: `config` 크레이트 사용 (프로젝트 루트 `config/default.toml`, `config/test.toml` 파일 활용)
        - 배포 관련 설정: `deploy/.env` 파일 사용 (Docker Compose 및 Nginx entrypoint에서 사용)
        - 주요 설정 항목 (TOML 기준): `log_level`, `server.port`, `server.host`, `server.api_version`
        - 주요 설정 항목 (.env 기준): `LOG_LEVEL`, `ROCKET_ADDRESS`, `ROCKET_PORT`, `ROCKET_API_ENDPOINT`, `NGINX_HTTP_PORT`, `NGINX_HTTPS_PORT` 등 배포 환경에 필요한 값
    * 시간 처리: `chrono` 크레이트 사용 (필요시 `serde` 기능 활성화)
  * **Frontend (Yew)** (향후 추가 예정)
    * 프로그래밍 언어: Rust
    * 웹 프레임워크: Yew v0.21
    * 상태 관리: Yew의 Hooks API 사용 (`use_state`, `use_effect_with` 등)
    * UI 라이브러리: Bootstrap 5 (CDN)
    * HTTP 클라이언트: `gloo-net` 크레이트 사용
    * 로깅: `wasm-logger` 및 `gloo-console` 크레이트 사용 (브라우저 콘솔 출력)
    * 빌드 도구: Trunk
  * **Deployment & Infrastructure**
    * 컨테이너 오케스트레이션: Docker Compose (`deploy/docker-compose.yml` 사용)
      * 관리 대상 서비스: `api`, `nginx`
      * 환경 변수 주입: `deploy/.env` 파일을 참조하여 각 서비스에 환경 변수 전달 (`env_file` 속성 사용)
      * 네트워크: `frand-api-network` (bridge 드라이버)
      * 볼륨:
          - `deploy/nginx/nginx.conf.template` -> `/etc/nginx/conf.d/nginx.conf.template` (읽기 전용, `nginx` 서비스)
          - `deploy/nginx/template_replace.sh` -> `/etc/nginx/template_replace.sh` (`nginx` 서비스)
          - `deploy/certs` -> `/etc/nginx/certs` (읽기 전용, TLS 인증서, `nginx` 서비스)
          - `deploy/static` -> `/usr/share/nginx/static` (읽기 전용, Nginx 정적 파일 서빙용, `nginx` 서비스)
      * 서비스 커맨드:
          - `nginx`: `/bin/sh /etc/nginx/template_replace.sh` 실행 (Nginx 설정 적용 및 실행)
    * 웹 서버 / 리버스 프록시: Nginx (`nginx` 서비스 내에서 실행)
      * 역할: API 서버(`api` 서비스)로의 리버스 프록시 (`${ROCKET_API_ENDPOINT}/`), 정적 파일 서빙 (`/static/`), HTTPS 처리
      * HTTPS 설정: `deploy/certs/` 경로에 볼륨 마운트된 `cert.pem`, `privkey.pem` 파일 사용. HTTP 요청은 HTTPS로 리다이렉션.
      * Nginx 설정 동적 적용: `template_replace.sh` 스크립트가 컨테이너 시작 시 `.env` 파일의 환경 변수를 참조하여 `nginx.conf.template`에 값을 적용, 최종 설정 생성.
      * 정적 파일 서빙: `deploy/static` 폴더를 볼륨 마운트하여 `/usr/share/nginx/static/` 경로로 서빙.
    * **Docker Base Images**
      * Rust Build (API): `rust:1.86-slim` (Multi-stage 빌드 활용)
      * API Runtime: `debian:bookworm-slim`
      * Nginx Runtime: `nginx:1.27.4-alpine-slim`
      * Database: `mongo:6.0` (향후 추가 예정)

## 기능
  * **Backend (API)**
    * 초기 엔드포인트: `/api/v1/health` (Nginx를 통해 접근 시 `https://<your-domain>/api/v1/health`)
      - GET 요청 시 JSON 형식으로 `{ "status": 200, "version": "<api_version_from_config>" }` 응답 반환. (`api_version_from_config`은 `config/default.toml`의 `server.api_version` 값)
      - 향후 데이터베이스 연결 상태 등 추가 정보 포함 예정.
  * **Nginx**
    * API 서버로 요청 프록시
    * HTTP 요청을 HTTPS로 자동 리다이렉션
    * `/static/` 경로로 정적 파일 서빙
  * **Frontend (Yew)** (향후 추가 예정)
    * 초기 기능: 루트 경로(`/`)에서 백엔드의 `/api/v1/health` 엔드포인트를 호출하여 상태 표시.
    * Bootstrap 기반의 반응형 UI 제공
    * API 오류 시 적절한 오류 메시지 표시

## 테스트 전략
  * **Backend (API)**
    * 통합 테스트 위주로 진행
    * Rocket의 `LocalClient`를 활용한 통합 테스트 고려 (`api/tests` 디렉토리)
  * **Frontend (Yew)**
    * 컴포넌트 단위 테스트 (향후)
    * E2E 테스트 (향후, `wasm-bindgen-test` 활용)

## API 버전 관리
  * URL 경로에 버전 포함 방식 사용.
  * API 버전 경로는 설정 파일 (프로젝트 루트 `config/default.toml`)의 `server.api_version` 항목에서 관리 (예: `/api/v1/...`)

## 향후 추가 예정
  * 데이터베이스: MongoDB
    - ODM 없이 `mongodb` 드라이버와 `serde` 직접 사용 예정
  * 인증/인가: Google OAuth, JWT
  * 초기 데이터 모델 (예시):
    - `User` 모델: `id` (고유 식별자), `username` (사용자 이름), `email` (이메일 주소) 등의 기본 필드 포함 (구체적인 타입은 추후 결정)
  * CI/CD: GitHub Actions
    - 자동 테스트, 빌드 및 DockerHub 배포 파이프라인 구성
  * 추가 UI 기능:
    - 사용자 인증 및 프로필 관리
    - 데이터 CRUD 작업을 위한 인터페이스
  * 모니터링:
    - 서버 상태 모니터링 및 로깅
    - 성능 지표 수집 및 시각화