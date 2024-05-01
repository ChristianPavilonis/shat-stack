use axum::{
    extract,
    response::Html,
    routing::{get, post},
    Router,
};
use components::ShatStack;
use layouts::Layout;
use shtml::{html, Component, Elements, Render};
use tower_http::services::ServeDir;

mod components;
mod layouts;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(home))
        .route("/shat/:count", post(shat))
        .fallback_service(ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2222").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<String> {
    let stack = vec![
        ("shtml", "https://github.com/swlkr/shtml"),
        ("htmx", "https://htmx.org/"),
        ("axum", "https://github.com/tokio-rs/axum"),
        ("tailwind", "https://tailwindcss.com/"),
    ];

    let stack = stack
        .iter()
        .map(|(name, url)| {
            html! {
                <li class="text-blue-500 text-2xl uppercase underline">
                    <a href=url> {name} </a>
                </li>
            }
        })
        .collect::<Vec<Component>>();

    let result = html! {
        <Layout>
            <div class="pt-24">
                <h1 class="text-5xl mb-24">
                    Welcome to the SHAT STACK
                </h1>

                <div class="flex justify-between max-w-[500px]">
                    <ShatStack count=0/>
                    <ul>
                        {stack}
                    </ul>
                </div>
            </div>
        </Layout>
    }
    .to_string();

    Html(result)
}

async fn shat(extract::Path(count): extract::Path<u16>) -> Html<String> {
    let result = html! {
        <ShatStack count=count />
    };

    Html(result.to_string())
}
