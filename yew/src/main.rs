use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{"Frand App"}</h1>
            <p>{"Welcome to the Frand application!"}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
