# 배포 (Deploy)

이 디렉토리에는 frand-api 애플리케이션을 Docker를 사용하여 배포하기 위한 파일들이 포함되어 있습니다. Nginx를 리버스 프록시로 사용하여 API 서버에 대한 HTTPS 접근 및 정적 파일 서빙을 지원하며, MongoDB 데이터베이스를 함께 구성합니다.

## 파일 설명

*   `api.dockerfile`: API 서버를 빌드하기 위한 Dockerfile입니다. 멀티 스테이지 빌드를 사용하여 최종 이미지 크기를 최적화합니다. (`common` 크레이트 포함)
*   `nginx.dockerfile`: Yew 프론트엔드를 빌드하고, Nginx 이미지를 생성하여 정적 파일 및 API 프록시를 서빙하기 위한 Dockerfile입니다. (`yew`, `common` 크레이트 포함)
*   `docker-compose.yml`: Docker Compose를 사용하여 `api`, `nginx`, `mongo` 서비스를 쉽게 실행하고 관리하기 위한 설정 파일입니다.
*   `.env.example`: 필요한 환경 변수 예시 파일입니다. 실제 배포 시에는 이 파일을 복사하여 `.env` 파일을 생성하고 값을 설정해야 합니다. (MongoDB 접속 정보 포함)
*   `nginx/`: Nginx 관련 설정 파일 및 스크립트
    *   `nginx.conf.template`: Nginx 설정 템플릿 파일입니다. 환경 변수를 사용하여 동적으로 설정됩니다.
    *   `template_replace.sh`: 컨테이너 시작 시 `.env` 파일의 환경 변수를 `nginx.conf.template`에 적용하고 Nginx를 실행하는 스크립트입니다.
*   `certs/`: TLS 인증서 파일을 저장하는 디렉토리입니다.
    *   `gen_certs.sh`: 로컬 개발 및 테스트용 자체 서명 TLS 인증서(`cert.pem`, `privkey.pem`)를 생성하는 스크립트입니다. **주의: 프로덕션 환경에서는 신뢰할 수 있는 인증 기관(CA)에서 발급받은 인증서를 사용해야 합니다.**
*   `static/`: Nginx를 통해 서빙될 정적 파일(예: 프론트엔드 빌드 결과물, 이미지 등)을 위치시키는 디렉토리입니다.
    *   **참고:** Yew 프론트엔드 빌드 결과물은 `nginx.dockerfile` 내에서 빌드되어 이미지에 포함되므로, 이 디렉토리에 직접 넣을 필요는 없습니다. 이 디렉토리는 추가적인 정적 파일(예: favicon.ico)을 위해 사용될 수 있습니다.

## 실행 방법

### Docker Compose 사용 (권장)

1.  **`.env` 파일 생성 및 설정:** `deploy/.env.example` 파일을 복사하여 `deploy/.env` 파일을 생성하고, 필요한 환경 변수 (`ROCKET_PORT`, `ROCKET_API_ENDPOINT`, `MONGO_*`, `NGINX_HTTP_PORT`, `NGINX_HTTPS_PORT` 등)를 설정합니다.
2.  **(최초 실행 또는 로컬 테스트 시) TLS 인증서 생성:** `deploy/certs` 디렉토리에서 다음 명령어를 실행하여 자체 서명 인증서를 생성합니다.
    ```bash
    ./gen_certs.sh
    ```
    *   이 스크립트는 `cert.pem`과 `privkey.pem` 파일을 생성합니다.
    *   프로덕션 환경에서는 이 단계를 건너뛰고, 발급받은 인증서 파일을 `deploy/certs` 디렉토리에 위치시킵니다.
3.  **Docker Compose 실행:** `deploy` 디렉토리에서 다음 명령어를 실행합니다.

    ```bash
    docker-compose up -d --build
    ```
    *   `-d`: 백그라운드에서 실행합니다. Yew 프론트엔드는 `http://localhost:${NGINX_HTTP_PORT}` 또는 `https://localhost:${NGINX_HTTPS_PORT}` 로 접근할 수 있습니다.
    *   `--build`: 이미지를 새로 빌드합니다. (최초 실행 또는 Dockerfile 변경 시 필요)
    *   이제 `http://localhost:${NGINX_HTTP_PORT}` 또는 `https://localhost/${NGINX_HTTPS_PORT}` 로 접근할 수 있습니다. (HTTP는 HTTPS로 자동 리다이렉션됩니다.) API는 `https://localhost:${NGINX_HTTPS_PORT}${ROCKET_API_ENDPOINT}/` 경로로 접근 가능합니다.

4.  **서비스 중지:**

    ```bash
    docker-compose down
    ```

### Docker 직접 사용

API 서버, Nginx, MongoDB를 각각 빌드하고 실행해야 하며, 네트워크 설정 및 환경 변수 전달이 복잡하므로 **Docker Compose 사용을 강력히 권장합니다.**

## 환경 변수

`deploy/.env` 파일에서 다음 환경 변수를 설정할 수 있습니다.

*   **API Service:**
    *   `LOG_LEVEL`: API 서버 로그 레벨 (예: "info", "debug")
    *   `ROCKET_ADDRESS`: Rocket 서버가 바인딩할 주소 (Docker 내부 통신용, 보통 "0.0.0.0")
    *   `ROCKET_PORT`: Rocket 서버가 리스닝할 포트 (Docker 내부 통신용, 예: 8080)
    *   `ROCKET_API_ENDPOINT`: API 기본 경로 (예: "/api/v1")
*   **MongoDB Service:**
    *   `MONGO_HOST`: MongoDB 호스트 이름 (Docker Compose 서비스 이름, 예: "mongo")
    *   `MONGO_PORT`: MongoDB 포트 (예: 27017)
    *   `MONGO_DB_NAME`: 사용할 데이터베이스 이름
    *   `MONGO_USER`: MongoDB 사용자 이름
    *   `MONGO_PASSWORD`: MongoDB 비밀번호
*   **NGINX Service:**
    *   `NGINX_HTTP_PORT`: 외부에서 HTTP로 접근할 포트 (예: 80)
    *   `NGINX_HTTPS_PORT`: 외부에서 HTTPS로 접근할 포트 (예: 443)

**주의:** `.env` 파일은 민감한 정보를 포함할 수 있으므로 Git 저장소에 직접 커밋하지 않도록 주의해야 합니다. (`.env.example` 파일을 대신 사용하고, 프로젝트 루트의 `.gitignore`에 `.env`가 포함되어 있는지 확인하세요.)
