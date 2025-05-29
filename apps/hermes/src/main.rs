mod db;
mod web;

use config::Config;
use shuttle_runtime::SecretStore;
use web::{App, AppConfig};

#[shuttle_runtime::main]
async fn axum(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let config = Config::builder()
        .add_source(config::File::with_name("Config.toml"))
        .build()
        .unwrap()
        .try_deserialize::<AppConfig>()
        .unwrap();

    let app = App::create(config, secrets).await;

    Ok(app.router().into())
}
