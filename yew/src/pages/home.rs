use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="home-page">
            <h1>{"Frand API - 메모 애플리케이션"}</h1>
            <p>{"Rust와 Yew로 만든 간단한 메모 애플리케이션입니다."}</p>
            
            <div class="home-links">
                <Link<Route> to={Route::MemoList} classes="btn btn-primary">
                    {"메모 목록 보기"}
                </Link<Route>>
            </div>
        </div>
    }
}
