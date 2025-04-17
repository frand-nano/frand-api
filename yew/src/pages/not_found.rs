use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div class="not-found-page">
            <h1>{"404 - 페이지를 찾을 수 없습니다"}</h1>
            <p>{"요청하신 페이지는 존재하지 않습니다."}</p>
            <Link<Route> to={Route::Home} classes="btn btn-primary">
                {"홈으로 돌아가기"}
            </Link<Route>>
        </div>
    }
}
