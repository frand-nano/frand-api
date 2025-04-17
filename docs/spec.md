# 기술 명세
## 프로젝트 정보
  * 라이선스: MIT
  * 초기 버전: 0.1.0
  * 개발자: frand-nano <frand.nano@gmail.com>
## 프로젝트 구조
  * `api/src` 내부에 `routes`, `handlers`, `models`, `services` 등의 모듈 구조 사용
## 의존성
  * 프로그래밍 언어: Rust
  * 웹 프레임워크: Rocket
  * 로깅: `log`, `simple_logger` 크레이트 사용 (터미널 출력)
  * 오류 처리: `anyhow`, `thiserror` 사용.
    - (추천 방식) `thiserror`로 사용자 정의 오류 타입을 정의하고, Rocket의 `Responder`를 구현하여 오류를 적절한 HTTP 응답으로 변환하는 방식 고려.
  * 설정 관리: `config` 크레이트 사용
  * 데이터베이스 드라이버 (향후 사용): `mongodb`
  * 직렬화/역직렬화 (향후 사용): `serde` (MongoDB 연동 시)
## 기능
  * 초기 엔드포인트: `/health`
    - 단순 상태 코드 반환 외에 향후 데이터베이스 연결 상태 등 추가 정보 포함 예정.
## 테스트 전략
  * 통합 테스트 위주로 진행
  * Rocket의 `LocalClient`를 활용한 통합 테스트 고려
## API 버전 관리
  * URL 경로에 버전 포함 방식 사용 (예: `/api/v1/...`)
## 향후 추가 예정
  * 데이터베이스: MongoDB
    - ODM 없이 `mongodb` 드라이버와 `serde` 직접 사용 예정
  * 인증/인가: Google OAuth, JWT
  * 초기 데이터 모델 (예시):
    - `User` 모델: `id` (고유 식별자), `username` (사용자 이름), `email` (이메일 주소) 등의 기본 필드 포함 (구체적인 타입은 추후 결정)
  * CI/CD: GitHub Actions