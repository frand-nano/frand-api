use crate::types::{ApiResponse, MemoData, MemoFrontend};
use gloo::net::http::{Request, Response};
use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

// API 엔드포인트 기본 경로 설정
fn api_endpoint() -> String {
    option_env!("FRONTEND_API_ENDPOINT")
        .unwrap_or("/api/v1")
        .to_string()
}

/// 메모 API 관련 상태와 기능을 제공하는 커스텀 훅
#[hook]
pub fn use_memo_api() -> (
    UseStateHandle<Vec<MemoFrontend>>,
    UseStateHandle<Option<MemoFrontend>>,
    UseStateHandle<bool>,
    UseStateHandle<Option<String>>,
    Callback<()>,
    Callback<String>,
    Callback<MemoData>,
    Callback<(String, MemoData)>,
    Callback<String>,
) {
    let memos = use_state(Vec::new);
    let memo = use_state(|| None);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    // 메모 목록 조회
    let fetch_list = {
        let memos = memos.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let memos = memos.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            loading.set(true);
            error.set(None);
            
            let endpoint = format!("{}/memos", api_endpoint());
            
            spawn_local(async move {
                match Request::get(&endpoint).send().await {
                    Ok(response) => {
                        if response.status() == 200 {
                            match response.json::<ApiResponse<Vec<MemoFrontend>>>().await {
                                Ok(data) => {
                                    info!("메모 목록 조회 성공: {} 개", data.data.len());
                                    memos.set(data.data);
                                }
                                Err(e) => {
                                    error!("메모 목록 JSON 파싱 오류: {:?}", e);
                                    error.set(Some("메모 데이터 파싱 중 오류가 발생했습니다.".to_string()));
                                }
                            }
                        } else {
                            handle_error_response(response, error).await;
                        }
                    }
                    Err(e) => {
                        error!("메모 목록 조회 요청 오류: {:?}", e);
                        error.set(Some("네트워크 오류가 발생했습니다.".to_string()));
                    }
                }
                loading.set(false);
            });
        })
    };

    // 단일 메모 조회
    let fetch_one = {
        let memo = memo.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |id: String| {
            let memo = memo.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            loading.set(true);
            error.set(None);
            
            let endpoint = format!("{}/memos/{}", api_endpoint(), id);
            
            spawn_local(async move {
                match Request::get(&endpoint).send().await {
                    Ok(response) => {
                        if response.status() == 200 {
                            match response.json::<ApiResponse<MemoFrontend>>().await {
                                Ok(data) => {
                                    info!("메모 조회 성공: {}", data.data.title);
                                    memo.set(Some(data.data));
                                }
                                Err(e) => {
                                    error!("단일 메모 JSON 파싱 오류: {:?}", e);
                                    error.set(Some("메모 데이터 파싱 중 오류가 발생했습니다.".to_string()));
                                }
                            }
                        } else {
                            handle_error_response(response, error).await;
                        }
                    }
                    Err(e) => {
                        error!("단일 메모 조회 요청 오류: {:?}", e);
                        error.set(Some("네트워크 오류가 발생했습니다.".to_string()));
                    }
                }
                loading.set(false);
            });
        })
    };

    // 메모 생성
    let create = {
        let memo = memo.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |data: MemoData| {
            let memo = memo.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            loading.set(true);
            error.set(None);
            
            let endpoint = format!("{}/memos", api_endpoint());
            
            spawn_local(async move {
                match Request::post(&endpoint)
                    .json(&data)
                    .expect("JSON 직렬화 실패")
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status() == 201 || response.status() == 200 {
                            match response.json::<ApiResponse<MemoFrontend>>().await {
                                Ok(data) => {
                                    info!("메모 생성 성공: {}", data.data.title);
                                    memo.set(Some(data.data));
                                }
                                Err(e) => {
                                    error!("메모 생성 응답 파싱 오류: {:?}", e);
                                    error.set(Some("응답 데이터 파싱 중 오류가 발생했습니다.".to_string()));
                                }
                            }
                        } else {
                            handle_error_response(response, error).await;
                        }
                    }
                    Err(e) => {
                        error!("메모 생성 요청 오류: {:?}", e);
                        error.set(Some("네트워크 오류가 발생했습니다.".to_string()));
                    }
                }
                loading.set(false);
            });
        })
    };

    // 메모 수정
    let update = {
        let memo = memo.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |(id, data): (String, MemoData)| {
            let memo = memo.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            loading.set(true);
            error.set(None);
            
            let endpoint = format!("{}/memos/{}", api_endpoint(), id);
            
            spawn_local(async move {
                match Request::put(&endpoint)
                    .json(&data)
                    .expect("JSON 직렬화 실패")
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status() == 200 {
                            match response.json::<ApiResponse<MemoFrontend>>().await {
                                Ok(data) => {
                                    info!("메모 수정 성공: {}", data.data.title);
                                    memo.set(Some(data.data));
                                }
                                Err(e) => {
                                    error!("메모 수정 응답 파싱 오류: {:?}", e);
                                    error.set(Some("응답 데이터 파싱 중 오류가 발생했습니다.".to_string()));
                                }
                            }
                        } else {
                            handle_error_response(response, error).await;
                        }
                    }
                    Err(e) => {
                        error!("메모 수정 요청 오류: {:?}", e);
                        error.set(Some("네트워크 오류가 발생했습니다.".to_string()));
                    }
                }
                loading.set(false);
            });
        })
    };

    // 메모 삭제
    let delete = {
        let loading = loading.clone();
        let error = error.clone();
        let memos = memos.clone();

        Callback::from(move |id: String| {
            let loading = loading.clone();
            let error = error.clone();
            let memos = memos.clone();
            
            loading.set(true);
            error.set(None);
            
            let endpoint = format!("{}/memos/{}", api_endpoint(), id);
            let id_clone = id.clone();
            
            spawn_local(async move {
                match Request::delete(&endpoint).send().await {
                    Ok(response) => {
                        if response.status() == 200 {
                            info!("메모 삭제 성공: ID {}", id_clone);
                            // 삭제된 메모를 목록에서 제거
                            memos.set(
                                memos
                                    .iter()
                                    .filter(|m| m.id.as_ref() != Some(&id_clone))
                                    .cloned()
                                    .collect(),
                            );
                        } else {
                            handle_error_response(response, error).await;
                        }
                    }
                    Err(e) => {
                        error!("메모 삭제 요청 오류: {:?}", e);
                        error.set(Some("네트워크 오류가 발생했습니다.".to_string()));
                    }
                }
                loading.set(false);
            });
        })
    };

    (
        memos,
        memo,
        loading,
        error,
        fetch_list,
        fetch_one,
        create,
        update,
        delete,
    )
}

// 오류 응답 처리 헬퍼 함수
async fn handle_error_response(response: Response, error: UseStateHandle<Option<String>>) {
    let status = response.status();
    match response.json::<serde_json::Value>().await {
        Ok(json) => {
            if let Some(error_msg) = json
                .get("error")
                .and_then(|e| e.get("message"))
                .and_then(|m| m.as_str())
            {
                error.set(Some(error_msg.to_string()));
            } else {
                error.set(Some(format!("오류 발생 (상태 코드: {})", status)));
            }
        }
        Err(_) => {
            error.set(Some(format!("오류 발생 (상태 코드: {})", status)));
        }
    }
}
