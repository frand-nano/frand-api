# frand-api

이 프로젝트는 Rust와 Rocket 프레임워크를 사용하여 구축된 API 서버입니다.

## 디렉토리 구조

*   `api/`: Rocket 기반의 API 서버 소스 코드 및 관련 파일이 위치합니다. 자세한 내용은 [`./api/README.md`](./api/README.md) 파일을 참고하세요.
*   `yew/`: Yew 기반의 프론트엔드 애플리케이션 소스 코드 및 관련 파일이 위치합니다. 자세한 내용은 [`./yew/README.md`](./yew/README.md) 파일을 참고하세요.
*   `common/`: API와 Yew 프론트엔드 간에 공유되는 공통 코드 라이브러리입니다.
*   `docs/`: 프로젝트 관련 문서 (기술 명세 등)가 위치합니다.
*   `deploy/`: Docker를 사용한 배포 관련 파일 (Dockerfile, docker-compose.yml 등)이 위치합니다. 자세한 내용은 [`./deploy/README.md`](./deploy/README.md) 파일을 참고하세요.

## 라이선스

이 프로젝트는 [`./LICENSE`](./LICENSE) 하에 배포됩니다.
