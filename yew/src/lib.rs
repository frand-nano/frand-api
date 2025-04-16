use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

// API 엔드포인트 환경변수 접근 (컴파일 타임), 없을 경우 기본값 사용
fn api_endpoint() -> String {
    option_env!("FRONTEND_API_ENDPOINT").unwrap_or("http://localhost:8080/api").to_string()
}

// 라우터 정의
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// 메인 애플리케이션 구조체
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="container">
                <header class="header">
                    <h1>{"Frand API Web"}</h1>
                </header>
                
                <main>
                    <Switch<Route> render={switch} />
                </main>
                
                <footer class="footer">
                    <p>{"© 2023 Frand API"}</p>
                </footer>
            </div>
        </BrowserRouter>
    }
}

// 라우트 핸들링 함수
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::NotFound => html! { <h1>{"404 페이지를 찾을 수 없습니다"}</h1> },
    }
}

// 홈페이지 컴포넌트
#[function_component(HomePage)]
fn home_page() -> Html {
    let health_status = use_state(|| None::<String>);
    let status = health_status.clone();
    
    let onclick = {
        Callback::from(move |_| {
            let status = status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let endpoint = format!("{}/health", api_endpoint());
                match gloo::net::http::Request::get(&endpoint).send().await {
                    Ok(response) => {
                        if response.status() == 200 {
                            match response.json::<serde_json::Value>().await {
                                Ok(data) => {
                                    if let Some(health_data) = data.get("data") {
                                        if let Some(health_status) = health_data.get("status") {
                                            status.set(Some(health_status.to_string()));
                                            return;
                                        }
                                    }
                                    status.set(Some("응답 형식 오류".to_string()));
                                },
                                Err(_) => status.set(Some("JSON 파싱 오류".to_string())),
                            }
                        } else {
                            status.set(Some(format!("에러: {}", response.status())));
                        }
                    },
                    Err(_) => status.set(Some("API 연결 실패".to_string())),
                }
            });
        })
    };

    html! {
        <div class="home-page">
            <h2>{"Frand API 웹 인터페이스"}</h2>
            <p>{"이 페이지는 Rust와 Yew로 구현된 웹 인터페이스입니다."}</p>
            
            <button class="btn" {onclick}>{"헬스 체크"}</button>
            
            {
                if let Some(status) = (*health_status).clone() {
                    html! { <p>{"서버 상태: "}{status}</p> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::Renderer::<App>::new().render();
    Ok(())
}
