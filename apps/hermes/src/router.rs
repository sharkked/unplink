use std::sync::Arc;

use crate::{AppConfig, cache::Cache, db::MongoConnection};
use axum::{
    Json, Router,
    extract::Path,
    http::{HeaderValue, StatusCode},
    response::Redirect,
    routing::{get, post},
};
use nanoid::nanoid;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use shuttle_runtime::SecretStore;
use tower::ServiceBuilder;
use tower_http::{
    auth::AsyncRequireAuthorizationLayer,
    cors::{Any, CorsLayer},
};
use url::Url;

#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

type HttpResult<T, E = StatusCode> = axum::response::Result<T, E>;
type JsonResult<T, E = StatusCode> = axum::response::Result<Json<T>, E>;

#[derive(Serialize)]
struct ShortenResponse {
    url: String,
}

impl ShortenResponse {
    fn ok(url: &String) -> JsonResult<Self> {
        Ok(Json(Self { url: url.into() }))
    }
}

async fn shorten(
    Json(payload): Json<ShortenRequest>,
    state: Arc<AppState>,
) -> JsonResult<ShortenResponse> {
    if !Url::parse(&payload.url).is_ok() {
        return Err(StatusCode::BAD_REQUEST);
    }
    // @TODO: deny own redirects
    let uid = nanoid!(10, &nanoid::alphabet::SAFE); // @TODO generate UID
    // match state.cache.set::<String>(&uid, &payload.url) {
    match state.db.set_url(&uid, &payload.url).await {
        Ok(()) => ShortenResponse::ok(&format!("{}/{uid}", state.base_url)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn redirect(Path(path): Path<String>, state: Arc<AppState>) -> HttpResult<Redirect> {
    // match state.cache.get::<String>(&path) {
    match state.db.find_url(&path).await {
        Ok(url) => Ok(Redirect::permanent(&url[..])),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

struct AppState {
    base_url: String,
    cache: Cache,
    db: MongoConnection,
}

pub async fn create(config: AppConfig, secrets: SecretStore) -> Router {
    let state = Arc::new(AppState {
        base_url: config.base_url,
        cache: Cache::open(
            &secrets
                .get("redis_uri")
                .expect("redis connection string was not found"),
        )
        .unwrap(),
        db: MongoConnection::open(
            &secrets
                .get("mongo_uri")
                .expect("mongo connection string was not found"),
        )
        .await
        .unwrap(),
    });

    Router::new()
        .route(
            "/",
            post({
                let shared_state = Arc::clone(&state);
                move |body| shorten(body, shared_state)
            }),
        )
        .route(
            "/{path}",
            get({
                let shared_state = Arc::clone(&state);
                move |body| redirect(body, shared_state)
            }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(AsyncRequireAuthorizationLayer::new(
                    crate::auth::TokenAuth {},
                ))
                .layer(
                    CorsLayer::new()
                        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                        .allow_methods(Any)
                        .allow_origin(config.allow_origin.parse::<HeaderValue>().unwrap()),
                ),
        )
}
