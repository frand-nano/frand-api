# Frand API 배포

Docker Compose를 사용하여 Frand API와 Yew 프론트엔드를 배포하기 위한 설정 파일들입니다.

## 디렉토리 구조

```
deploy/
├── docker-compose.yml       # Docker Compose 설정 파일
├── nginx/                   # Nginx 설정 파일
│   ├── Dockerfile           # Nginx + Yew 프론트엔드 빌드 설정
│   ├── conf.d/              # Nginx 설정
│   │   └── default.conf     # 기본 서버 설정 (환경변수 템플릿)
│   └── docker-entrypoint.sh # Nginx 시작 스크립트 (환경변수 처리)
├── secrets/                 # 민감 정보 (gitignore)
│   └── certs/               # TLS 인증서 디렉토리
├── .env.example             # 환경 변수 예시
└── generate-certs.sh        # TLS 인증서 생성 스크립트
```

## 배포 설정

### 환경 변수

주요 환경 변수:
- `NGINX_DOMAIN`: 서버 도메인 이름 (기본값: localhost)
- `TLS_CERT_FILE`: TLS 인증서 파일명 (기본값: cert.pem)
- `TLS_KEY_FILE`: TLS 개인키 파일명 (기본값: privkey.pem)
- `ROCKET_PORT`: API 서버 포트 (기본값: 8000)
- `ROCKET_ADDRESS`: API 서버 바인딩 주소 (기본값: 0.0.0.0)

### TLS 인증서

개발/테스트용 자체 서명 인증서 생성:
```
./generate-certs.sh
```

인증서는 `secrets/certs/` 디렉토리에 생성됩니다.
배포 환경에서는 유효한 인증서로 교체하는 것을 권장합니다.

### Nginx 설정

- 80 포트(HTTP)와 443 포트(HTTPS)에서 리스닝
- `/api/` 경로는 API 서비스로 프록시
- 루트(`/`) 경로는 Yew 프론트엔드 정적 파일 제공
- SPA(단일 페이지 애플리케이션) 라우팅을 위한 설정 포함

## 배포 방법

1. 환경 변수 설정:
   ```
   cp .env.example .env
   # .env 파일을 필요에 맞게 수정
   ```

2. TLS 인증서 생성 또는 배치:
   ```
   ./generate-certs.sh
   # 또는 유효한 인증서를 직접 secrets/certs/ 디렉토리에 복사
   ```

3. Docker Compose를 사용하여 서비스 시작:
   ```
   docker compose up -d
   ```

4. 서비스 상태 확인:
   ```
   docker compose ps
   ```

5. 로그 확인:
   ```
   docker compose logs
   ```

6. 서비스 중지:
   ```
   docker compose down
   ```
