// 테스트 환경 설정을 위한 공통 모듈

pub fn setup() {
    dotenvy::from_filename(".env.test").ok();
}
