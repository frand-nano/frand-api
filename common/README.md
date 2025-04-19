# 공통 라이브러리 (frand-api-common)

이 크레이트는 `frand-api` 워크스페이스 내의 다른 크레이트들(주로 `api`와 `yew`) 간에 공유되는 코드들을 모아놓은 라이브러리입니다. 코드 중복을 방지하고 일관성을 유지하는 것을 목표로 합니다.

## 구조

*   `src/models/`: API와 프론트엔드 간에 주고받는 데이터 전송 객체(DTO) 등 공통 데이터 구조를 정의합니다.
    *   `user.rs`: 사용자 관련 DTO (`UserDto`, `CreateUserDto`) 정의

## 주요 의존성

*   `serde`: 데이터 직렬화/역직렬화를 위해 사용됩니다.

## 사용법

워크스페이스 내의 다른 크레이트의 `Cargo.toml` 파일에 다음과 같이 의존성을 추가하여 사용할 수 있습니다.

```toml
[dependencies]
frand-api-common = { path = "../common" }
```
