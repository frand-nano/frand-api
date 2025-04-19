# Frand API Yew 프론트엔드

Frand API 서비스의 Yew 기반 프론트엔드 애플리케이션입니다.

## 개발 환경 설정

Yew 개발을 위해서는 [Trunk](https://trunkrs.dev/)가 필요합니다.

```bash
cargo install trunk wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
```

## 빌드

프로젝트 루트 디렉토리에서 다음 명령어를 실행하여 Yew 애플리케이션을 빌드합니다. 빌드 결과물은 `yew/dist` 디렉토리에 생성됩니다.

```bash
trunk build --release
```

## 개발 서버 실행

개발 중에는 다음 명령어를 사용하여 개발 서버를 실행할 수 있습니다. 변경 사항이 감지되면 자동으로 리빌드됩니다.

```bash
trunk serve
```

기본적으로 `http://localhost:8000`에서 애플리케이션을 확인할 수 있습니다.
