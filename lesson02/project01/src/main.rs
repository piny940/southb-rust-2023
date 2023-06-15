use std::collections::HashMap;

use axum::{extract::Path, response::IntoResponse, routing::get, Extension, Router};

/// 保存された値を取得する。ただし、値が存在しない場合は 404 を返す。
async fn get_by_key(
    Extension(data): Extension<HashMap<String, String>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    todo!()
}

/// 保存された値を更新する。
async fn post_by_key(
    Extension(mut data): Extension<HashMap<String, String>>,
    Path(key): Path<String>,
    body: String,
) -> impl IntoResponse {
    todo!()
}

fn make_router() -> Router {
    Router::new().route("/key/:id", get(get_by_key).post(post_by_key))
}

#[tokio::main]
async fn main() {
    let app = make_router().layer(Extension(HashMap::<String, String>::new()));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use hyper::{body::Bytes, Method};
    use tower::ServiceExt;

    fn init_test_router() -> axum::Router {
        use super::*;
        make_router().layer(Extension(HashMap::<String, String>::new()))
    }

    #[tokio::test]
    async fn test_nonexistent_key() -> Result<()> {
        let router = init_test_router();
        let response = router
            .oneshot(Request::builder().uri("/key/1").body(Body::empty())?)
            .await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body: Bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"");
        Ok(())
    }

    #[tokio::test]
    async fn test_put_and_get() -> Result<()> {
        let mut router = init_test_router();
        let router = router.ready().await?;

        // insert 1 → test_content_1
        let response = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/key/1")
                    .body("test_content_1".into())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        // insert alpha → test_content_alpha
        let response = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/key/alpha")
                    .body("test_content_alpha".into())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        // query beta
        let response = router
            .oneshot(Request::builder().uri("/key/beta").body(Body::empty())?)
            .await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body: Bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"");

        // query 1
        let response = router
            .oneshot(Request::builder().uri("/key/beta").body(Body::empty())?)
            .await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body: Bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"");

        // query alpha
        let response = router
            .oneshot(Request::builder().uri("/key/beta").body(Body::empty())?)
            .await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body: Bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"");

        Ok(())
    }
}
