use crate::models::Item;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ItemListProps {
    pub items: Vec<Item>,
    pub on_delete: Callback<String>,
    pub on_edit: Callback<String>,
}

#[function_component(ItemList)]
pub fn item_list(props: &ItemListProps) -> Html {
    let on_delete = |id: String| {
        let on_delete = props.on_delete.clone();
        Callback::from(move |_| {
            on_delete.emit(id.clone());
        })
    };

    let on_edit = |id: String| {
        let on_edit = props.on_edit.clone();
        Callback::from(move |_| {
            on_edit.emit(id.clone());
        })
    };

    html! {
        <div class="item-list">
            <h2>{"아이템 목록"}</h2>
            {if props.items.is_empty() {
                html! { <p>{"등록된 아이템이 없습니다."}</p> }
            } else {
                html! {
                    <table>
                        <thead>
                            <tr>
                                <th>{"제목"}</th>
                                <th>{"내용"}</th>
                                <th>{"작업"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {props.items.iter().map(|item| {
                                let id = item.id.clone().unwrap_or_default();
                                html! {
                                    <tr key={id.to_string()}>
                                        <td>{&item.title}</td>
                                        <td>{&item.message}</td>
                                        <td>
                                            <button onclick={on_edit(id.clone())}>{"수정"}</button>
                                            <button onclick={on_delete(id.clone())}>{"삭제"}</button>
                                        </td>
                                    </tr>
                                }
                            }).collect::<Html>()}
                        </tbody>
                    </table>
                }
            }}
        </div>
    }
}
