use crate::router::Route;
use crate::types::MemoFrontend;
use chrono::{DateTime, Local, Utc};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MemoListItemProps {
    pub memo: MemoFrontend,
    pub on_delete: Callback<String>,
}

#[function_component(MemoListItem)]
pub fn memo_list_item(props: &MemoListItemProps) -> Html {
    let memo = &props.memo;
    let on_delete = props.on_delete.clone();
    
    // 수정일시 표시 형식화
    let formatted_date = memo
        .updated_at
        .or(memo.created_at)
        .map(|date| format_date(date))
        .unwrap_or_else(|| "날짜 정보 없음".to_string());
    
    // 내용 미리보기 (최대 100자)
    let content_preview = if memo.content.len() > 100 {
        format!("{}...", &memo.content[..97])
    } else {
        memo.content.clone()
    };
    
    // 삭제 버튼 클릭 핸들러
    let on_delete_click = {
        let id = memo.id.clone().unwrap_or_default();
        let on_delete = on_delete.clone();
        
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if web_sys::window()
                .unwrap()
                .confirm_with_message("이 메모를 삭제하시겠습니까?")
                .unwrap()
            {
                on_delete.emit(id.clone());
            }
        })
    };

    html! {
        <div class="memo-card">
            <div class="memo-header">
                <h3 class="memo-title">{&memo.title}</h3>
                <div class="memo-actions">
                    <Link<Route> to={Route::MemoEdit { id: memo.id.clone().unwrap_or_default() }} classes="edit-btn">
                        {"수정"}
                    </Link<Route>>
                    <button class="delete-btn" onclick={on_delete_click}>{"삭제"}</button>
                </div>
            </div>
            <p class="memo-content">{content_preview}</p>
            <small class="memo-date">{"최종 수정: "}{formatted_date}</small>
        </div>
    }
}

// 날짜 형식화 헬퍼 함수
fn format_date(date: DateTime<Utc>) -> String {
    let local_time = date.with_timezone(&Local);
    local_time.format("%Y년 %m월 %d일 %H:%M").to_string()
}
