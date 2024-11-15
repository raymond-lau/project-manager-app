use axum::{extract::Path, response::{Html, IntoResponse}, routing::get, Router};
use serde::{Deserialize, Serialize};
use axum::{ Json, http::StatusCode};
use serde_json::json;

#[derive(Deserialize, Serialize)]
struct MyAppState {
    db_connection: PgPool,
}
async fn get_project(Path(id): Path<i32>
) -> impl IntoResponse {
    let string = format!("Hello world {}!", id);

    (StatusCode::OK, string)
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route(
        "/:id", 
        get(get_project)
    );

    Ok(router.into())
}
