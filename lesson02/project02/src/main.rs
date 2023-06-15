use axum::{extract::Path, response::IntoResponse, routing::get, Router};

/// 保存された値を取得する。ただし、値が存在しない場合は 404 を返す。
/// ただしキーは必ず有効なファイルネームである。
async fn get_by_key(Path(key): Path<String>) -> impl IntoResponse {
    todo!()
}

/// 保存された値を更新する。
/// ただしキーは必ず有効なファイルネームである。
async fn post_by_key(Path(key): Path<String>, body: String) -> impl IntoResponse {
    todo!()
}

fn make_router() -> Router {
    Router::new().route("/key/:id", get(get_by_key).post(post_by_key))
}

#[tokio::main]
async fn main() {
    let app = make_router();

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
        make_router()
    }

    #[tokio::test]
    async fn test_nonexistent_key() -> Result<()> {
        let router = init_test_router();
        let uuid = ::uuid::Uuid::new_v4().to_string();
        let response = router
            .oneshot(
                Request::builder()
                    .uri(&format!("/key/{uuid}"))
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body: Bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"");
        Ok(())
    }

    #[tokio::test]
    async fn test_put_and_get() -> Result<()> {
        let uuid1 = ::uuid::Uuid::new_v4().to_string();
        let uuid2 = ::uuid::Uuid::new_v4().to_string();

        // insert uuid1 → test_content_1
        let response = init_test_router()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/key/{uuid1}"))
                    .body("test_content_1".into())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        // query uuid1
        let response = init_test_router()
            .oneshot(
                Request::builder()
                    .uri(format!("/key/{uuid1}"))
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"test_content_1");

        // query uuid2
        let response = init_test_router()
            .oneshot(
                Request::builder()
                    .uri(format!("/key/{uuid2}"))
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body: Bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"");

        Ok(())
    }
}
