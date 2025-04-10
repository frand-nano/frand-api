# Frand API

Rust로 개발된 경량 API 서버 프로젝트입니다.

## 주의
docs/guide 문서를 이용하여 AI 로 생성된 연습용 프로토타입 프로젝트입니다.
대부분의 코드가 검증되지 않았으며, 아래와 같은 여러 가지 보안 요소가 아직 미구현 상태입니다.

사용자 인증 시스템
역할 기반 접근 제어
API 요청 속도 제한
데이터 유효성 검사
MongoDB 보안 설정 (접근 제한, TLS 설정)
API 보안 헤더 추가
HTTP에서 HTTPS로 강제 리디렉션 설정

## 주요 기능

- Rocket 웹 프레임워크 기반 API 서버
- MongoDB 데이터베이스 연동
- Docker 기반 배포 환경 설정 (Nginx, MongoDB)
- RESTful API 엔드포인트 구현

## 기술 스택

- **언어:** Rust
- **웹 프레임워크:** Rocket
- **데이터베이스:** MongoDB
- **배포:** Docker, Nginx

## API 엔드포인트

### 루트 (Root)
- `GET /`: 서버 상태 확인

### 아이템 (Items)
- `POST /items`: 새 아이템 생성
- `GET /items`: 모든 아이템 목록 조회
- `GET /items/<id>`: 특정 ID의 아이템 조회
- `PUT /items/<id>`: 특정 ID의 아이템 수정
- `DELETE /items/<id>`: 특정 ID의 아이템 삭제

## 프로젝트 구조

```
frand-api/
├── api/           # API 서비스 코드
├── deploy/        # Docker 배포 관련 파일
│   ├── nginx/     # Nginx 설정 파일
│   └── secrets/   # TLS 인증서 등 민감 정보 저장 (gitignore)
├── docs/          # 프로젝트 문서
├── yew/           # Yew 프론트엔드 애플리케이션
└── target/        # 빌드 결과물
```

## 개발 환경 설정

### 필수 조건
- Rust와 Cargo 설치
- Docker와 Docker Compose 설치 (배포용)
- MongoDB (로컬 개발용)

### 환경 변수 설정

1. API 서버 환경 설정:
   ```
   cp api/.env.example api/.env
   ```

2. Yew 프론트엔드 환경 설정:
   ```
   cp yew/.env.example yew/.env
   ```

3. 배포 환경 설정:
   ```
   cp deploy/.env.example deploy/.env
   ```

4. 테스트를 위한 환경 설정:
   ```
   cp api/.env.test.example api/.env.test
   ```

5. 필요에 따라 각 환경 변수 값 수정

### 개발 환경 실행

API 서버 실행:
```
cargo run -p api
```

Yew 프론트엔드 개발 서버 실행 (Trunk 필요):
```
cd yew && trunk serve
```

### Docker 배포

TLS 인증서 생성:
```
cd deploy && ./generate-certs.sh
```

Docker Compose로 실행:
```
cd deploy && docker compose up -d
```

### 테스트

테스트 실행:
```
cargo test -p api
```

## 라이선스

MIT 라이선스로 제공됩니다. 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.
