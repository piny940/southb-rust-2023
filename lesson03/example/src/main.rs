#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool =
        sqlx::PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await?;

    let items = sqlx::query!("SELECT * FROM items").fetch_all(&pool).await?;
    println!("{:?}", &items);

    Ok(())
}
