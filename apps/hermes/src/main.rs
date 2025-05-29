mod users;
mod web;

use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use web::App;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let app = App::create(pool, secrets).await.unwrap();
    Ok(app.router().await.unwrap().into())
}
