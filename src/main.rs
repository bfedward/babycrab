use sqlx::postgres::PgPoolOptions;

mod db_client;
mod error;
mod github_client;
mod harvest_issues;
mod model;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Could not dotenvy");

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
    {
        Ok(pool) => pool,
        Err(e) => panic!("Couldn't establish DB connection: {}", e),
    };

    if let Err(err) =
        harvest_issues::harvest_issues(std::env::var("GITHUB_TOKEN").unwrap(), &pool).await
    {
        println!("Harvest failed: {err:#?}");
    }
}
