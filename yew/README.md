# Frand Yew 프론트엔드

Rust의 Yew 프레임워크를 사용한 웹 프론트엔드 애플리케이션입니다. WebAssembly로 컴파일되어 실행됩니다.

## 디렉토리 구조

```
yew/
├── src/
│   └── main.rs       # 애플리케이션 진입점 및 컴포넌트
├── index.html        # HTML 템플릿
├── style.css         # CSS 스타일시트
├── Cargo.toml        # Cargo 패키지 설정
├── trunk.toml        # Trunk 빌드 설정
└── .env.example      # 환경 변수 예시
```

## 요구사항

- Rust 및 Cargo (1.86.0 이상 권장)
- Trunk - Yew 애플리케이션 빌드 도구
- wasm32-unknown-unknown 대상 (WebAssembly 빌드용)

## 개발 환경 설정

1. WebAssembly 대상 추가:
   ```
   rustup target add wasm32-unknown-unknown
   ```

2. Trunk 설치:
   ```
   cargo install trunk
   ```

3. 환경 변수 설정:
   ```
   cp .env.example .env
   # 필요에 따라 .env 파일 수정
   ```

## 개발 서버 실행

```
trunk serve
```

이 명령은 개발 서버를 시작하고 기본적으로 http://127.0.0.1:8080 에서 접근할 수 있습니다.
파일이 변경되면 자동으로 다시 빌드되고 브라우저가 갱신됩니다.

## 빌드

개발용 빌드:
```
trunk build
```

배포용 빌드:
```
trunk build --release
```

빌드 결과물은 `dist` 디렉토리에 생성됩니다.

## API 연동

프론트엔드는 `/api/` 경로를 통해 백엔드 API와 통신합니다.
API 기본 URL은 `.env` 파일의 `API_BASE_URL` 환경 변수로 설정할 수 있습니다.

## Docker 배포

프로젝트 루트의 `deploy` 디렉토리에 있는 Docker Compose 설정을 통해 
Nginx 컨테이너 내에 빌드된 프론트엔드 애플리케이션이 포함되어 배포됩니다.
