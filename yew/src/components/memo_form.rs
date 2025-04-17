use crate::types::MemoData;
use validator::ValidationErrors;
use validator::Validate;
use web_sys::HtmlInputElement;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MemoFormProps {
    pub initial_data: MemoData,
    pub on_submit: Callback<MemoData>,
}

#[function_component(MemoForm)]
pub fn memo_form(props: &MemoFormProps) -> Html {
    let form_data = use_state(|| props.initial_data.clone());
    let validation_errors = use_state(|| None::<ValidationErrors>);
    
    // 제목 입력 핸들러
    let on_title_change = {
        let form_data = form_data.clone();
        
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut data = (*form_data).clone();
            data.title = input.value();
            form_data.set(data);
        })
    };
    
    // 내용 입력 핸들러
    let on_content_change = {
        let form_data = form_data.clone();
        
        Callback::from(move |e: Event| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            let mut data = (*form_data).clone();
            data.content = textarea.value();
            form_data.set(data);
        })
    };
    
    // 폼 제출 핸들러
    let on_submit = {
        let form_data = form_data.clone();
        let validation_errors = validation_errors.clone();
        let props_on_submit = props.on_submit.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let data = (*form_data).clone();
            
            // 유효성 검사
            match data.validate() {
                Ok(_) => {
                    validation_errors.set(None);
                    props_on_submit.emit(data);
                }
                Err(errors) => {
                    validation_errors.set(Some(errors));
                }
            }
        })
    };
    
    // 필드별 오류 메시지 추출 헬퍼 함수
    let get_field_error = |field: &str| -> Option<String> {
        validation_errors.as_ref().and_then(|errors| {
            errors.field_errors().get(field).map(|error_list| {
                error_list
                    .first()
                    .and_then(|error| error.message.as_ref().map(|m| m.to_string()))
                    .unwrap_or_else(|| format!("{} 필드에 오류가 있습니다.", field))
            })
        })
    };

    html! {
        <form class="memo-form" onsubmit={on_submit}>
            <div class="form-group">
                <label for="title">{"제목"}</label>
                <input 
                    type="text" 
                    id="title" 
                    name="title" 
                    value={form_data.title.clone()} 
                    onchange={on_title_change} 
                    class={classes!("form-control", get_field_error("title").is_some().then_some("is-invalid"))}
                />
                {
                    if let Some(error) = get_field_error("title") {
                        html! { <div class="error-message">{error}</div> }
                    } else {
                        html! {}
                    }
                }
            </div>
            
            <div class="form-group">
                <label for="content">{"내용"}</label>
                <textarea 
                    id="content" 
                    name="content" 
                    rows="5" 
                    value={form_data.content.clone()} 
                    onchange={on_content_change}
                    class={classes!("form-control", get_field_error("content").is_some().then_some("is-invalid"))}
                />
                {
                    if let Some(error) = get_field_error("content") {
                        html! { <div class="error-message">{error}</div> }
                    } else {
                        html! {}
                    }
                }
            </div>
            
            <div class="form-actions">
                <button type="submit" class="btn btn-primary">{"저장"}</button>
            </div>
        </form>
    }
}
