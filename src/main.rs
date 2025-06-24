mod users;
mod web;

use std::env;

use dotenv::dotenv;
use sqlx::PgPool;
use web::App;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DB_URI").unwrap()).await.unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    let app = App::create(pool).await.unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    let router = app.router().await.unwrap();
    axum::serve(listener, router).await.unwrap()
}
