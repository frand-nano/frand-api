// 테스트 환경 설정을 위한 공통 모듈

// 테스트 환경 설정 함수
pub fn setup() {
    // 테스트용 .env.test 파일 로드
    dotenvy::from_filename(".env.test").ok();
}
