# Frand API

Rust 기반 REST API 서버와 Yew 기반 웹 애플리케이션입니다.

## 프로젝트 구조

본 프로젝트는 Rust 워크스페이스로 구성되어 있으며, 다음과 같은 패키지 및 디렉토리를 포함합니다.

*   `api/`: 핵심 REST API 서버 구현체 (Rocket 프레임워크 사용).
*   `yew/`: Yew 프레임워크 기반 웹 애플리케이션 구현체.
*   `deploy/`: Docker 기반 배포 관련 파일 (Dockerfiles, docker-compose.yml, Nginx 설정, 인증서 생성 스크립트 등).
*   `docs/`: 프로젝트 관련 문서.
*   `.env.example`: 필요한 환경 변수 예시 파일 (`DATABASE_NAME` 포함).
*   `.dockerignore`: Docker 빌드 시 제외할 파일 목록.

## 실행 방법 (Docker Compose 사용)

1.  **`.env` 파일 설정:** `deploy` 디렉토리에 `.env.example` 파일을 복사하여 `.env` 파일을 생성하고 필요한 환경 변수(데이터베이스 사용자/비밀번호, 포트, 데이터베이스 이름 등)를 설정합니다.
    ```bash
    cd deploy
    cp .env.example .env
    # nano .env 또는 다른 편집기로 .env 파일 수정
    ```
2.  **TLS 인증서 준비:** `./deploy/secure/tls/` 디렉토리에 `cert.pem`과 `privkey.pem` 파일을 위치시킵니다. 개발/테스트 환경에서는 `deploy/gen_certs.sh` 스크립트를 사용하여 자체 서명 인증서를 생성할 수 있습니다. (OpenSSL 필요)
    ```bash
    # deploy 디렉토리에서 실행
    bash gen_certs.sh
    ```
3.  **Docker Compose 실행:**
    ```bash
    # deploy 디렉토리에서 실행
    docker-compose up -d --build
    ```
    이제 `https://localhost` (또는 설정된 도메인)으로 접속하여 웹 애플리케이션을, `/api/v1/health` 또는 `/api/v1/memos` 등으로 API를 확인할 수 있습니다.

## 테스트 실행

```bash
# API 서버 테스트
cd api
cargo test

# Yew Frontend 테스트 (현재 구현되지 않음)
# cd yew
# cargo test
```
API 테스트 실행 전 프로젝트 루트에 `.env.test` 파일을 필요에 따라 설정하십시오. (`DATABASE_NAME` 포함)

## 주의
docs/guide 문서를 이용하여 AI 로 생성된 연습용 프로토타입 프로젝트입니다.
대부분의 코드가 검증되지 않았으며, 아래와 같은 여러 가지 보안 요소가 아직 미구현 상태입니다.

*   사용자 인증 시스템
*   역할 기반 접근 제어
*   API 요청 속도 제한
*   데이터 유효성 검사
*   MongoDB 보안 설정 (접근 제한, TLS 설정)
*   API 보안 헤더 추가
*   HTTP에서 HTTPS로 강제 리디렉션 설정

## 상세 정보

프로젝트의 자세한 설계 및 구현 명세는 [`docs/`](./docs/) 디렉토리의 명세 문서들을 참고하십시오.
