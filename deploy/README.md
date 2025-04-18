# 배포 (Deploy)

이 디렉토리에는 frand-api 애플리케이션을 Docker를 사용하여 배포하기 위한 파일들이 포함되어 있습니다.

## 파일 설명

*   `api.dockerfile`: API 서버를 빌드하기 위한 Dockerfile입니다. 멀티 스테이지 빌드를 사용하여 최종 이미지 크기를 최적화합니다.
*   `docker-compose.yml`: Docker Compose를 사용하여 API 서비스를 쉽게 실행하고 관리하기 위한 설정 파일입니다.
*   `.env`: Docker Compose 및 애플리케이션 실행에 필요한 환경 변수를 정의하는 파일입니다. 이 파일은 컨테이너 내부로 복사되어 애플리케이션 설정에 사용됩니다.

## 실행 방법

### Docker Compose 사용 (권장)

1.  **`.env` 파일 설정:** `deploy/.env` 파일에 필요한 환경 변수 (예: `ROCKET_PORT`)를 설정합니다.
2.  **Docker Compose 실행:** `deploy` 디렉토리에서 다음 명령어를 실행합니다.

    ```bash
    docker-compose up -d --build
    ```
    *   `-d`: 백그라운드에서 실행합니다.
    *   `--build`: 이미지를 새로 빌드합니다. (최초 실행 또는 Dockerfile 변경 시 필요)

3.  **서비스 중지:**

    ```bash
    docker-compose down
    ```

### Docker 직접 사용

1.  **이미지 빌드:** 프로젝트 루트 디렉토리에서 다음 명령어를 실행합니다.

    ```bash
    docker build -t frand-api:latest -f deploy/api.dockerfile .
    ```

2.  **컨테이너 실행:**

    ```bash
    docker run -d --name frand-api-container --env-file deploy/.env -p <호스트_포트>:${ROCKET_PORT} frand-api:latest
    ```
    *   `<호스트_포트>`를 외부에서 접속할 포트로 변경합니다. (예: `8080`)
    *   `${ROCKET_PORT}`는 `deploy/.env` 파일에 정의된 포트 번호입니다.

## 환경 변수

`deploy/.env` 파일에서 다음 환경 변수를 설정할 수 있습니다.

*   `LOG_LEVEL`: 애플리케이션 로그 레벨 (예: "info", "debug")
*   `ROCKET_ADDRESS`: Rocket 서버가 바인딩할 주소 (예: "0.0.0.0")
*   `ROCKET_PORT`: Rocket 서버가 리스닝할 포트 (예: 8080)
*   `ROCKET_API_ENDPOINT`: API 기본 경로 (예: "/api/v1")

**주의:** `.env` 파일은 민감한 정보를 포함할 수 있으므로 Git 저장소에 직접 커밋하지 않도록 주의해야 합니다. (프로젝트 루트의 `.gitignore`에 이미 포함되어 있습니다.)
