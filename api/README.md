# Frand API 서비스

Rust로 작성된 API 서버 모듈입니다. Rocket 웹 프레임워크를 사용합니다.

## 디렉토리 구조

```
api/
├── src/
│   ├── lib.rs         # 라이브러리 루트, Rocket 인스턴스 생성 함수 포함
│   ├── main.rs        # 애플리케이션 진입점
│   ├── config.rs      # 애플리케이션 설정
│   ├── models/        # 데이터 모델
│   ├── routes/        # API 라우트 핸들러
│   └── services/      # 비즈니스 로직
├── tests/             # 통합 테스트
│   └── common/        # 테스트 공통 유틸리티
├── Dockerfile         # Docker 이미지 빌드 파일
├── .env.example       # 환경 변수 예시
└── .env.test.example  # 테스트 환경 변수 예시
```

## API 엔드포인트

현재 구현된 엔드포인트:

- `GET /`: "hello world" 메시지 반환

## 환경 변수

주요 환경 변수:
- `ROCKET_PORT`: 서버 포트 (기본값: 8000)
- `ROCKET_ADDRESS`: 바인딩할 주소 (기본값: "0.0.0.0")

## Docker 배포

API 서비스는 멀티스테이지 Docker 빌드 방식으로 배포됩니다:
- 빌드 스테이지에서 Rust 컴파일러를 사용해 바이너리 생성
- 실행 스테이지에서 경량 Debian 이미지에 바이너리 복사하여 실행

배포는 루트 프로젝트의 `deploy` 디렉토리에 있는 Docker Compose 설정을 통해 관리됩니다.

## 개발하기

### 서버 실행

```
cargo run
```

### 테스트 실행

```
cargo test
```

## 확장 계획

- 데이터베이스 연동
- 사용자 인증/인가
- 추가 API 엔드포인트
