use axum::Router;

fn make_router() -> Router {
    Router::new()
}

#[tokio::main]
async fn main() {
    let app = make_router();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
