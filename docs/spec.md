# Frand API 기술 명세서

## 1. 개요

본 문서는 `frand-api` 프로젝트의 기술적인 설계와 구현 명세를 기술합니다. 이 프로젝트는 Rust 기반의 REST API 서버와 Yew 프레임워크 기반의 웹 프론트엔드로 구성된 풀스택 웹 애플리케이션 프로토타입입니다. Docker Compose를 사용하여 개발 및 배포 환경을 관리합니다.

## 2. 기술 스택

*   **백엔드 (API)**:
    *   언어: Rust
    *   웹 프레임워크: Rocket (`0.5`)
    *   데이터베이스 드라이버: `mongodb`
*   **프론트엔드 (Web App)**:
    *   언어: Rust
    *   프레임워크: Yew (`0.21`)
    *   빌드 도구/개발 서버: Trunk
    *   라우팅: `yew-router` (`0.18`)
    *   Web API 연동: `wasm-bindgen`, `web-sys`, `gloo`
    *   스타일링: CSS (`static/style.css`)
*   **데이터베이스**:
    *   MongoDB
*   **배포**:
    *   컨테이너화: Docker, Docker Compose
    *   웹 서버/리버스 프록시: Nginx
*   **개발 환경**:
    *   Rust Toolchain (Cargo)

## 3. 프로젝트 구조

```
.
├── api/                  # Rust Rocket API 서버 소스 코드
│   ├── Cargo.toml
│   └── src/
├── deploy/               # 배포 관련 파일 (Docker, Nginx)
│   ├── docker-compose.yml
│   ├── api.Dockerfile
│   ├── yew.Dockerfile
│   ├── nginx/
│   │   └── nginx.conf.template
│   ├── .env.example      # 배포 환경 변수 예시
│   └── gen_certs.sh      # 개발용 TLS 인증서 생성 스크립트
├── docs/                 # 프로젝트 문서
│   └── spec.md           # 기술 명세서 (이 파일)
├── yew/                  # Yew 프론트엔드 소스 코드
│   ├── Cargo.toml
│   ├── Trunk.toml        # Trunk 빌드/개발 서버 설정
│   ├── index.html        # 기본 HTML 템플릿
│   ├── static/           # 정적 에셋 (CSS 등)
│   └── src/
├── .env.example          # 프로젝트 루트 환경 변수 예시 (주로 Yew 빌드용)
├── .dockerignore         # Docker 빌드 제외 목록
├── Cargo.toml            # Rust 워크스페이스 설정
└── README.md             # 프로젝트 개요 및 사용법
```

## 4. API 서버 (`api` 패키지)

*   **프레임워크**: Rocket (`0.5`)
*   **주요 기능**: RESTful API 엔드포인트 제공
*   **데이터베이스 연동**: `mongodb` 크레이트를 사용하여 MongoDB와 상호작용합니다. 데이터베이스 연결 정보는 환경 변수를 통해 설정됩니다.
*   **설정**: 주로 환경 변수(`.env` 또는 시스템 환경 변수)를 통해 데이터베이스 연결 정보 등을 설정합니다.
*   **테스트**: `cargo test` 명령어를 통해 단위 및 통합 테스트를 실행합니다. 테스트 시 별도의 `.env.test` 파일을 사용할 수 있습니다. (`DATABASE_NAME` 등)

## 5. 프론트엔드 (`yew` 패키지)

*   **프레임워크**: Yew (`0.21`)
*   **주요 기능**: 사용자 인터페이스 제공, API 서버와 데이터 통신
*   **빌드 및 개발**: Trunk를 사용하여 애플리케이션을 빌드하고 개발 서버를 실행합니다.
    *   `Trunk.toml`: 빌드 경로, 프록시 설정, 환경 변수 주입 등을 정의합니다.
    *   개발 시 API 요청 (`/api/`)은 `Trunk.toml`의 프록시 설정을 통해 백엔드 API 서버로 전달됩니다.
*   **라우팅**: `yew-router` (`0.18`)를 사용하여 클라이언트 사이드 라우팅을 구현합니다.
*   **API 연동**: `gloo-net` 등을 사용하여 백엔드 API와 통신합니다. API 엔드포인트 경로는 빌드 시점에 `FRONTEND_API_ENDPOINT` 환경 변수를 통해 주입받습니다. 이 변수는 프로젝트 루트의 `.env` 파일 또는 Docker 빌드 인자를 통해 설정됩니다.
*   **정적 파일**: CSS, 이미지 등 정적 파일은 `static/` 디렉토리에 위치하며, Trunk 빌드 시 `dist/` 디렉토리로 복사됩니다.
*   **테스트**: 현재 프론트엔드 테스트 전략은 정의되지 않았습니다.

## 6. 배포 (`deploy` 디렉토리)

*   **방식**: Docker Compose를 사용하여 API 서버, 프론트엔드(Nginx 서빙), 데이터베이스 컨테이너를 함께 실행합니다.
*   **서비스 구성**:
    *   `api`: Rust API 서버 컨테이너 (`api.Dockerfile` 기반).
    *   `yew_frontend`: Nginx 컨테이너. Yew 앱 빌드 결과 정적 파일을 서빙하고, API 요청을 `api` 서비스로 프록시합니다 (`yew.Dockerfile` 기반).
    *   `mongo`: MongoDB 데이터베이스 컨테이너.
*   **Nginx 설정 (`nginx.conf.template`)**:
    *   HTTPS 리다이렉션 (80 -> 443).
    *   TLS 종료 (자체 서명 또는 실제 인증서 사용).
    *   API 요청 (`/api/v1/`)을 `api` 서비스로 프록시 패스.
    *   Yew 정적 파일 서빙 및 SPA 라우팅 지원 (`try_files`).
    *   정적 에셋 캐싱 (`/static/`).
    *   환경 변수를 사용하여 동적으로 설정 파일 생성.
*   **환경 변수**: `deploy/.env` 파일을 통해 `docker-compose.yml` 및 컨테이너 내부에서 사용할 환경 변수(예: `DATABASE_NAME`)를 설정합니다. `.env.example` 파일이 템플릿 역할을 합니다.
*   **TLS**: `gen_certs.sh` 스크립트를 사용하여 개발 및 테스트용 자체 서명 TLS 인증서를 생성할 수 있습니다. 실제 배포 시에는 유효한 인증서가 필요합니다.

## 7. 데이터베이스

*   **종류**: MongoDB
*   **데이터 모델**: (현재 코드베이스에서는 구체적인 스키마 정보가 부족합니다. 추후 정의 필요)
*   **관리**: Docker Compose를 통해 `mongo` 서비스로 실행됩니다. 데이터는 Docker 볼륨에 저장됩니다.

## 8. 환경 변수

주요 환경 변수는 다음과 같습니다.

*   **`DATABASE_NAME`** (`deploy/.env`, `.env.test`): API 서버 및 테스트에서 사용할 MongoDB 데이터베이스 이름.
*   **`FRONTEND_API_ENDPOINT`** (프로젝트 루트 `.env`, Docker 빌드 인자): Yew 프론트엔드가 API 요청 시 사용할 기본 경로 (예: `/api/v1`). `yew.Dockerfile` 빌드 시점에 주입됩니다.
*   **(기타 MongoDB 관련 변수)**: `MONGO_INITDB_ROOT_USERNAME`, `MONGO_INITDB_ROOT_PASSWORD` 등 (필요시 `docker-compose.yml` 및 `deploy/.env`에 정의).
*   **(기타 Rocket 관련 변수)**: `ROCKET_ADDRESS`, `ROCKET_PORT` 등 (필요시 설정).

## 9. 보안 고려 사항

현재 프로토타입 단계로, 다음과 같은 보안 요소들이 **미구현** 상태입니다. 실제 운영 환경 배포 전 반드시 보완해야 합니다.

*   사용자 인증 및 인가 시스템
*   역할 기반 접근 제어 (RBAC)
*   API 요청 속도 제한 (Rate Limiting)
*   입력 데이터 유효성 검사 강화
*   MongoDB 보안 강화 (네트워크 접근 제한, 사용자 인증, TLS 암호화 등)
*   보안 관련 HTTP 헤더 추가 (CSP, HSTS, X-Frame-Options 등)
*   HTTPS 강제 적용 및 설정 검증
*   의존성 보안 취약점 점검

## 10. 향후 개선 사항

*   프론트엔드 테스트 전략 수립 및 구현
*   상세한 데이터 모델 정의 및 문서화
*   로깅 및 모니터링 시스템 구축
*   CI/CD 파이프라인 구축
*   위에 명시된 보안 고려 사항 구현
