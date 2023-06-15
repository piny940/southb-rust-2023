// この関数を編集しないで
pub(crate) async fn slow_fn(x: i32) -> i32 {
    use std::time::Duration;
    tokio::time::sleep(Duration::from_secs(1)).await;
    x * x
}