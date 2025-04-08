# Frand API

Rust로 개발된 웹 API 서비스입니다. Rocket 프레임워크를 기반으로 구축되었습니다.

## 프로젝트 구조

```
frand-api/
├── api/           # API 서비스 코드
├── docs/          # 프로젝트 문서
└── target/        # 빌드 결과물
```

## 시작하기

### 요구사항

- Rust 및 Cargo (1.86.0 이상 권장)
- 환경 변수 설정 (아래 '환경 설정' 참조)

### 환경 설정

1. `.env.example` 파일을 복사하여 `.env` 파일 생성:
   ```
   cp api/.env.example api/.env
   ```

2. 테스트를 위한 환경 설정:
   ```
   cp api/.env.test.example api/.env.test
   ```

3. 필요에 따라 환경 변수 값 수정

### 실행

개발 서버 실행:
```
cargo run -p api
```

### 테스트

테스트 실행:
```
cargo test -p api
```

## 라이선스

MIT 라이선스로 제공됩니다. 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.
