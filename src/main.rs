use axum::{response::{Html, IntoResponse}, routing::get, Router};

async fn get_project() -> impl IntoResponse {
    Html("Project: <strong>world!</strong>")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route(
        "/", 
        get(get_project)
    );
//test
    Ok(router.into())
}
