use crate::pages::{
    home::HomePage,
    memo_edit::MemoEditPage,
    memo_list::MemoListPage,
    not_found::NotFoundPage,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/memos")]
    MemoList,
    #[at("/memos/new")]
    MemoCreate,
    #[at("/memos/:id/edit")]
    MemoEdit { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::MemoList => html! { <MemoListPage /> },
        Route::MemoCreate => html! { <MemoEditPage memo_id={None::<String>} /> },
        Route::MemoEdit { id } => html! { <MemoEditPage memo_id={Some(id)} /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}
