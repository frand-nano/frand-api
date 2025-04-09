# Frand API

Rust로 개발된 웹 애플리케이션입니다. Rocket 기반 API 서버와 Yew 기반 프론트엔드로 구성되어 있습니다.

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

## 시작하기

### 요구사항

- Rust 및 Cargo (1.86.0 이상 권장)
- Docker 및 Docker Compose (배포용)
- 환경 변수 설정 (아래 '환경 설정' 참조)

### 환경 설정

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
