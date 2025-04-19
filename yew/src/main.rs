use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {    
    html! {
        <div class="container mt-5">
            <h1>{"Frand API 프론트엔드"}</h1>
        </div>
    }
}