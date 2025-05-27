mod auth;
mod cache;
mod db;
mod router;

use config::Config;
use serde::Deserialize;
use shuttle_runtime::SecretStore;

#[derive(Deserialize)]
struct AppConfig {
    base_url: String,
    allow_origin: String,
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let config = Config::builder()
        .add_source(config::File::with_name("Config.toml"))
        .build()
        .unwrap()
        .try_deserialize::<AppConfig>()
        .unwrap();

    let router = router::create(config, secrets);

    Ok(router.await.into())
}
