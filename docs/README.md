# 문서 (`docs` 디렉토리)

이 디렉토리에는 `frand-api` 프로젝트의 설계, 구현 명세 및 관련 문서들이 포함되어 있습니다.

## 문서 목록

*   **[spec_01.md](./spec_01.md):** 초기 API 서버 (MVP) 설계 명세서입니다.
    *   Rust 및 Rocket 기반 API 서버의 기본 구조, 설정, 로깅, 오류 처리, 초기 엔드포인트(`/`, `/api/v1/health`) 등을 정의합니다.
    *   MongoDB, Docker 배포, Frontend 등은 향후 구현 기능으로 명시되어 있습니다.
*   **[spec_02.md](./spec_02.md):** Yew Frontend, Docker 기반 배포, MongoDB 연동을 포함한 확장된 시스템 설계 명세서입니다.
    *   `api`, `yew`, `deploy` 디렉토리 구조를 정의합니다.
    *   Docker Compose를 이용한 서비스 구성 (API, Yew/Nginx, MongoDB), Dockerfile 정의, Nginx 설정 (TLS, 프록시, 정적 파일 서빙) 등을 상세히 기술합니다.
    *   Yew Frontend 빌드 및 환경 변수 주입 방법 (Trunk, `FRONTEND_API_ENDPOINT`)을 설명합니다.
    *   MongoDB 연동을 위한 환경 변수 및 Docker 설정을 포함합니다.
    *   전체 시스템의 설정 관리, 데이터베이스, API 버전 관리, 오류 처리, 로깅, 테스트, 배포 전략 등을 종합적으로 다룹니다.
*   **[spec_03.md](./spec_03.md):** MongoDB 기반 메모 CRUD API 기능 명세서입니다.
    *   메모 데이터 모델(`Memo`), API 엔드포인트(`/api/v1/memos`), 요청/응답 형식, 데이터베이스 연동, 오류 처리, 테스트 케이스 등을 상세히 정의합니다.
*   **[spec_04.md](./spec_04.md):** Yew 프론트엔드 메모 CRUD 기능 명세서입니다.
    *   `spec_03.md`에서 정의된 API를 사용하는 Yew 애플리케이션의 라우팅, 컴포넌트 구조(페이지, 재사용 컴포넌트, 커스텀 훅), 데이터 모델, API 연동, 유효성 검사, 스타일링 등을 상세히 정의합니다.

(향후 프로젝트가 발전함에 따라 추가적인 설계 문서나 가이드가 이 디렉토리에 포함될 수 있습니다.)
