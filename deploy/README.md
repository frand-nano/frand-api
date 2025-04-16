# 배포 (`deploy` 디렉토리)

이 디렉토리에는 `frand-api` 프로젝트의 Docker 기반 배포를 위한 설정 파일들이 포함되어 있습니다.

## 포함된 파일

*   **`docker-compose.yml`**: API 서버 (`api`), Yew Frontend 및 Nginx (`yew_frontend`), MongoDB (`mongo`) 서비스를 정의하고 관리하는 Docker Compose 설정 파일입니다. 각 서비스의 빌드 방법, 환경 변수, 네트워크, 볼륨 등을 정의합니다. `api` 서비스의 `8080` 포트는 컨테이너 외부로 노출되어 디버깅 목적으로 접근할 수 있습니다.
*   **`api.Dockerfile`**: Rust API 서버를 빌드하고 실행하기 위한 Dockerfile입니다. 멀티 스테이지 빌드를 사용하여 최종 이미지 크기를 최적화합니다.
*   **`yew.Dockerfile`**: Yew Frontend 애플리케이션을 빌드하고, Nginx 서버를 사용하여 정적 파일을 서빙하기 위한 Dockerfile입니다. 멀티 스테이지 빌드를 사용하며, 빌드 시점에 `.env` 파일의 `FRONTEND_API_ENDPOINT` 값을 주입받습니다.
*   **`nginx/nginx.conf.template`**: `yew_frontend` 서비스의 Nginx 설정 파일 템플릿입니다. 컨테이너 시작 시 `docker-entrypoint.sh` 스크립트에 의해 환경 변수가 적용되어 최종 설정 파일(`default.conf`)이 생성됩니다. HTTPS 리다이렉션, TLS 종료, API 요청 프록시 (`/api/v1/` 경로), Yew 정적 파일 서빙 및 라우팅 지원 (`try_files`), 정적 에셋 캐싱 (`/static/`) 등을 설정합니다.
*   **`nginx/docker-entrypoint.sh`**: Nginx 컨테이너 시작 시 실행되는 스크립트입니다. `nginx.conf.template` 파일에 환경 변수를 적용하여 실제 Nginx 설정 파일을 생성하고 Nginx 를 실행합니다.
*   **`secure/tls/`**: TLS 인증서 파일 (`cert.pem`, `privkey.pem`)을 위치시키는 디렉토리입니다. 이 디렉토리의 내용은 Docker Compose를 통해 Nginx 컨테이너의 `/etc/nginx/certs` 경로로 마운트됩니다.
*   **`.env.example`**: `docker-compose.yml`에서 사용하는 환경 변수의 예시를 보여주는 파일입니다. 실제 배포 시 이 파일을 복사하여 `.env` 파일을 생성하고 값을 수정해야 합니다.
*   **`gen_certs.sh`**: 개발 및 테스트 환경에서 사용할 자체 서명 TLS 인증서를 생성하는 셸 스크립트입니다.

## 실행 방법

1.  **`.env` 파일 설정:** 이 `deploy` 디렉토리에 `.env.example` 파일을 복사하여 `.env` 파일을 생성하고, 필요한 환경 변수 (데이터베이스 사용자/비밀번호, 외부 포트 등)를 설정합니다.
    ```bash
    cp .env.example .env
    # nano .env 또는 다른 편집기로 .env 파일 수정
    ```
2.  **TLS 인증서 준비:** `deploy/secure/tls/` 디렉토리에 유효한 `cert.pem` (인증서)과 `privkey.pem` (개인 키) 파일을 위치시킵니다.
    *   **개발/테스트:** `gen_certs.sh` 스크립트를 실행하여 자체 서명 인증서를 생성할 수 있습니다. (OpenSSL 필요)
        ```bash
        # deploy 디렉토리에서 실행
        bash gen_certs.sh
        ```
    *   **운영:** 실제 도메인에 맞는 인증 기관(CA)에서 발급받은 인증서를 사용해야 합니다.
3.  **Docker Compose 실행:** `deploy` 디렉토리에서 다음 명령어를 실행합니다.
    ```bash
    docker-compose up -d --build
    ```
    *   `--build`: 이미지가 없거나 Dockerfile이 변경된 경우 이미지를 새로 빌드합니다.
    *   `-d`: 백그라운드에서 컨테이너를 실행합니다.

4.  **서비스 확인:**
    *   웹 애플리케이션: `https://localhost` (또는 `.env`에 설정된 `NGINX_EXTERNAL_PORT`가 443이 아닌 경우 해당 포트)
    *   API 상태 확인: `https://localhost/api/v1/health`
    *   API 직접 접근 (디버깅용): `http://localhost:8080/api/v1/health` (Nginx를 거치지 않음)

## 중지 및 제거

```bash
# 실행 중인 컨테이너 중지 및 제거
docker-compose down

# 볼륨(MongoDB 데이터)까지 완전히 제거하려면
docker-compose down -v
```

## 상세 정보

배포 구성 및 전체 시스템의 자세한 설계 및 구현 명세는 프로젝트 루트의 [docs/spec_02.md](../../docs/spec_02.md) 파일을 참고하십시오.
