use crate::components::memo_form::MemoForm;
use crate::hooks::use_memo_api::use_memo_api;
use crate::router::Route;
use crate::types::MemoData;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MemoEditPageProps {
    pub memo_id: Option<String>,
}

#[function_component(MemoEditPage)]
pub fn memo_edit_page(props: &MemoEditPageProps) -> Html {
    let navigator = use_navigator().unwrap();
    let (_, memo, loading, error, _, fetch_one, create, update, _) = use_memo_api();
    let success_message = use_state(|| None::<String>);
    
    // 수정 모드일 경우 메모 데이터 로드
    let is_edit_mode = props.memo_id.is_some();
    
    {
        let memo_id = props.memo_id.clone();
        let fetch_one = fetch_one.clone();
        
        use_effect_with(
            (),
            move |_| {
                if let Some(id) = memo_id {
                    fetch_one.emit(id);
                }
                || {}
            },
        );
    }
    
    // 폼 초기 데이터 설정
    let initial_data = if is_edit_mode {
        memo.as_ref().map(|m| MemoData::from(m)).unwrap_or_default()
    } else {
        MemoData::default()
    };
    
    // 폼 제출 핸들러
    let on_form_submit = {
        let memo_id = props.memo_id.clone();
        let create = create.clone();
        let update = update.clone();
        let success_message = success_message.clone();
        
        Callback::from(move |data: MemoData| {
            if let Some(id) = memo_id.clone() {
                // 수정 모드
                update.emit((id, data));
                success_message.set(Some("메모가 성공적으로 수정되었습니다.".to_string()));
            } else {
                // 생성 모드
                create.emit(data);
                success_message.set(Some("새 메모가 성공적으로 생성되었습니다.".to_string()));
            }
        })
    };
    
    // 목록으로 돌아가기 버튼 핸들러
    let on_back = {
        let navigator = navigator.clone();
        
        Callback::from(move |_| {
            navigator.push(&Route::MemoList);
        })
    };

    html! {
        <div class="memo-edit-page">
            <h1>
                if is_edit_mode {
                    {"메모 수정"}
                } else {
                    {"새 메모 작성"}
                }
            </h1>
            
            // 로딩 스피너
            if *loading {
                <div class="loading-spinner">
                    <div class="spinner"></div>
                    <p>{"로딩 중..."}</p>
                </div>
            }
            
            // 오류 메시지
            if let Some(err_msg) = error.as_ref() {
                <div class="error-message">
                    <p>{"오류 발생:"}</p>
                    <p>{err_msg}</p>
                </div>
            }
            
            // 성공 메시지
            if let Some(msg) = success_message.as_ref() {
                <div class="success-message">
                    <p>{msg}</p>
                </div>
            }
            
            // 메모 폼
            if !*loading || !is_edit_mode {
                <MemoForm initial_data={initial_data} on_submit={on_form_submit} />
            }
            
            <div class="form-actions">
                <button class="btn btn-secondary" onclick={on_back}>{"목록으로 돌아가기"}</button>
            </div>
        </div>
    }
}
