mod components;
mod hooks;
mod pages;
mod router;
mod types;

use router::{Route, switch};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app-container">
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

pub fn run_app() -> Result<(), JsValue> {
    yew::Renderer::<App>::new().render();
    Ok(())
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run() {
    run_app().expect("애플리케이션 실행 중 오류 발생");
}
