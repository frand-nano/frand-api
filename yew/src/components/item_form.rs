use crate::models::{Item, ItemData};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ItemFormProps {
    pub item_to_edit: Option<Item>,
    pub on_submit: Callback<ItemData>,
}

#[function_component(ItemForm)]
pub fn item_form(props: &ItemFormProps) -> Html {
    let title = use_state(|| String::new());
    let message = use_state(|| String::new());
    
    // 수정 모드일 때 폼 필드 초기화
    {
        let title = title.clone();
        let message = message.clone();
        
        use_effect_with(
            (props.item_to_edit.clone(),),
            move |(item_to_edit,)| {
                if let Some(item) = item_to_edit {
                    title.set(item.title.clone());
                    message.set(item.message.clone());
                } else {
                    title.set(String::new());
                    message.set(String::new());
                }
                || ()
            },            
        );
    }
    
    let on_title_change = {
        let title = title.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };
    
    let on_message_change = {
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            message.set(input.value());
        })
    };
    
    let on_submit = {
        let title = title.clone();
        let message = message.clone();
        let on_submit = props.on_submit.clone();
        let is_item_to_edit = props.item_to_edit.is_some();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // 간단한 유효성 검사
            if title.is_empty() {
                return;
            }
            
            let item_data = ItemData {
                title: (*title).clone(),
                message: (*message).clone(),
            };
            
            on_submit.emit(item_data);
            
            // 폼 초기화 (편집 모드가 아닐 때만)
            if !is_item_to_edit {
                title.set(String::new());
                message.set(String::new());
            }
        })
    };
    
    let form_title = if props.item_to_edit.is_some() {
        "아이템 수정"
    } else {
        "새 아이템 추가"
    };
    
    html! {
        <div class="item-form">
            <h2>{form_title}</h2>
            <form onsubmit={on_submit}>
                <div class="form-group">
                    <label for="title">{"제목:"}</label>
                    <input 
                        type="text" 
                        id="title" 
                        value={(*title).clone()} 
                        onchange={on_title_change}
                        required=true
                    />
                </div>
                <div class="form-group">
                    <label for="message">{"내용:"}</label>
                    <textarea 
                        id="message" 
                        value={(*message).clone()} 
                        onchange={on_message_change}
                    />
                </div>
                <div class="form-group">
                    <button type="submit">
                        {if props.item_to_edit.is_some() { "수정하기" } else { "추가하기" }}
                    </button>
                    {if props.item_to_edit.is_some() {
                        let on_submit = props.on_submit.clone();
                        let cancel = Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            // 빈 ItemData를 전송하여 편집 모드 취소 신호로 사용
                            on_submit.emit(ItemData { title: String::new(), message: String::new() });
                        });
                        
                        html! { <button onclick={cancel} type="button">{"취소"}</button> }
                    } else {
                        html! {}
                    }}
                </div>
            </form>
        </div>
    }
}
