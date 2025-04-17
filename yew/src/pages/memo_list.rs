use crate::components::memo_list_item::MemoListItem;
use crate::hooks::use_memo_api::use_memo_api;
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(MemoListPage)]
pub fn memo_list_page() -> Html {
    let (memos, _, loading, error, fetch_list, _, _, _, delete) = use_memo_api();
    
    // 컴포넌트 마운트 시 메모 목록 조회
    use_effect_with(
        (),
        move |_| {
            fetch_list.emit(());
            || {}
        },
    );

    html! {
        <div class="memo-list-page">
            <div class="memo-list-header">
                <h1>{"메모 목록"}</h1>
                <Link<Route> to={Route::MemoCreate} classes="btn btn-primary">
                    {"새 메모 작성"}
                </Link<Route>>
            </div>
            
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
            
            // 메모 목록
            <div class="memo-list">
                if memos.is_empty() && !*loading && error.is_none() {
                    <p class="no-memos">{"메모가 없습니다. 새 메모를 작성해보세요."}</p>
                } else {
                    {
                        memos.iter().map(|memo| {
                            let id = memo.id.clone().unwrap_or_default();
                            html! {
                                <MemoListItem 
                                    key={id.clone()} 
                                    memo={memo.clone()} 
                                    on_delete={delete.clone()} 
                                />
                            }
                        }).collect::<Html>()
                    }
                }
            </div>
        </div>
    }
}
