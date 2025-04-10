mod api;
mod components;
mod models;

use components::item_form::ItemForm;
use components::item_list::ItemList;
use models::{Item, ItemData};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

#[function_component(App)]
fn app() -> Html {
    let items = use_state(|| Vec::<Item>::new());
    let editing_item_id = use_state(|| Option::<String>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    // 초기 데이터 로드
    {
        let items = items.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with(
            (),
            move |_| {
                loading.set(true);
                error.set(None);

                spawn_local(async move {
                    match api::get_items().await {
                        Ok(fetched_items) => {
                            items.set(fetched_items);
                        }
                        Err(e) => {
                            let error_msg = format!("아이템 목록을 불러오는 중 오류 발생: {}", e);
                            console::error_1(&error_msg.clone().into());
                            error.set(Some(error_msg));
                        }
                    }
                    loading.set(false);
                });
                || ()
            }            
        );
    }

    // 아이템 삭제 콜백
    let on_delete = {
        let items = items.clone();
        let error = error.clone();

        Callback::from(move |id: String| {
            let items = items.clone();
            let error = error.clone();

            spawn_local(async move {
                match api::delete_item(&id).await {
                    Ok(_) => {
                        // 성공적으로 삭제된 경우 목록에서도 제거
                        let new_items = (*items)
                            .iter()
                            .filter(|item| item.id.as_ref() != Some(&id))
                            .cloned()
                            .collect::<Vec<_>>();
                        items.set(new_items);
                    }
                    Err(e) => {
                        let error_msg = format!("아이템 id:{id} 삭제 중 오류 발생: {}", e);
                        console::error_1(&error_msg.clone().into());
                        error.set(Some(error_msg));
                    }
                }
            });
        })
    };

    // 아이템 수정 모드 진입 콜백
    let on_edit = {
        let editing_item_id = editing_item_id.clone();

        Callback::from(move |id: String| {
            editing_item_id.set(Some(id));
        })
    };

    // 현재 수정 중인 아이템 조회
    let item_to_edit = {
        let items = items.clone();
        let editing_id = (*editing_item_id).clone();

        editing_id.and_then(|id| {
            items.iter().find(|item| item.id.as_ref() == Some(&id)).cloned()
        })
    };

    // 폼 제출 콜백 (생성 또는 수정)
    let on_submit = {
        let items = items.clone();
        let editing_item_id = editing_item_id.clone();
        let error = error.clone();

        Callback::from(move |data: ItemData| {
            // 빈 title이면 취소 신호로 간주
            if data.title.is_empty() {
                editing_item_id.set(None);
                return;
            }

            let items = items.clone();
            let editing_id = (*editing_item_id).clone();
            let editing_item_id = editing_item_id.clone();
            let error = error.clone();

            spawn_local(async move {
                let result = if let Some(id) = &editing_id {
                    // 수정 모드
                    api::update_item(id, &data).await.map(|updated_item| {
                        // 목록에서 해당 아이템 업데이트
                        let mut new_items = (*items).clone();
                        if let Some(idx) = new_items.iter().position(|item| item.id.as_ref() == Some(&id)) {
                            new_items[idx] = updated_item.clone();
                        }
                        new_items
                    })
                } else {
                    // 생성 모드
                    api::create_item(&data).await.map(|new_item| {
                        let mut new_items = (*items).clone();
                        new_items.push(new_item);
                        new_items
                    })
                };

                match result {
                    Ok(new_items) => {
                        items.set(new_items);
                        editing_item_id.set(None); // 편집 모드 종료
                    }
                    Err(e) => {
                        let action = if editing_id.is_some() { "수정" } else { "생성" };
                        let error_msg = format!("아이템 {} 중 오류 발생: {}, {:?}", action, e, data);
                        console::error_1(&error_msg.clone().into());
                        error.set(Some(error_msg));
                    }
                }
            });
        })
    };

    html! {
        <div class="app">
            <h1>{"Frand App"}</h1>
            
            // 에러 메시지 표시
            if let Some(err_msg) = (*error).clone() {
                <div class="error-message">
                    <p>{err_msg}</p>
                    <button onclick={let error = error.clone(); move |_| error.set(None)}>
                        {"닫기"}
                    </button>
                </div>
            }
            
            // 로딩 표시
            if *loading {
                <div class="loading">{"로딩 중..."}</div>
            } else {
                <>
                    <ItemForm item_to_edit={item_to_edit} on_submit={on_submit} />
                    <ItemList 
                        items={(*items).clone()} 
                        on_delete={on_delete} 
                        on_edit={on_edit} 
                    />
                </>
            }
        </div>
    }
}

fn main() {
    // 로거 초기화
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("애플리케이션 시작");
    
    yew::Renderer::<App>::new().render();
}
