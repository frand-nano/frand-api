# Frand API

Rust 기반 REST API 서버와 Yew 기반 웹 애플리케이션입니다.

## 프로젝트 구조

본 프로젝트는 Rust 워크스페이스로 구성되어 있으며, 다음과 같은 패키지 및 디렉토리를 포함합니다.

*   `api/`: 핵심 REST API 서버 구현체 (Rocket 프레임워크 사용).
*   `yew/`: Yew 프레임워크 기반 웹 애플리케이션 구현체.
*   `deploy/`: Docker 기반 배포 관련 파일 (Dockerfiles, docker-compose.yml, Nginx 설정 등).
*   `docs/`: 프로젝트 관련 문서.

## 실행 방법 (Docker Compose 사용)

1.  **.env 파일 설정:** 프로젝트 루트에 `.env` 파일을 생성하고 필요한 환경 변수(데이터베이스 사용자/비밀번호, 포트 등)를 설정합니다. `.env` 파일 예시는 `docs/spec_02.md` 문서를 참고하십시오.
2.  **TLS 인증서 준비:** `./deploy/secure/tls/` 디렉토리에 `cert.pem`과 `privkey.pem` 파일을 위치시킵니다. 개발/테스트 환경에서는 자체 서명 인증서를 생성하여 사용할 수 있습니다.
3.  **Docker Compose 실행:**
    ```bash
    cd deploy
    docker-compose up -d --build
    ```
    이제 `https://localhost` (또는 설정된 도메인)으로 접속하여 웹 애플리케이션을, `/api/v1/health` 등으로 API를 확인할 수 있습니다.

## 테스트 실행

```bash
# API 서버 테스트
cd api
cargo test

# Yew Frontend 테스트 (구현 필요)
# cd yew
# cargo test
```
테스트 실행 전 프로젝트 루트에 `.env.test` 파일을 필요에 따라 설정하십시오.

## 상세 정보

프로젝트의 자세한 설계 및 구현 명세는 다음 문서를 참고하십시오.

*   [docs/spec_01.md](docs/spec_01.md): 초기 API 서버 (MVP) 설계 명세
*   [docs/spec_02.md](docs/spec_02.md): Yew Frontend, Docker 배포, MongoDB 연동 설계 명세
