use axum::{
    Json, Router,
    extract::{Path, State},
    http::{
        HeaderValue, StatusCode,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    response::Redirect,
    routing::{get, post},
};
use axum_login::AuthManagerLayerBuilder;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_sessions::{ExpiredDeletion, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use url::Url;

use crate::users::Backend;

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
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> JsonResult<ShortenResponse> {
    if !Url::parse(&payload.url).is_ok() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // @TODO: deny own redirects
    // @TODO: collision detection
    let code = nanoid!(10, &nanoid::alphabet::SAFE);

    let query = sqlx::query("insert into shortlinks (url, code) values ($1, $2)")
        .bind(&payload.url)
        .bind(&code)
        .execute(&state.db)
        .await;

    match query {
        Ok(_) => ShortenResponse::ok(&format!("{}/{code}", std::env::var("BASE_URL").unwrap())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn redirect(Path(path): Path<String>, state: State<AppState>) -> HttpResult<Redirect> {
    let query = sqlx::query("select * from shortlinks where code = $1")
        .bind(&path)
        .fetch_optional(&state.db)
        .await;

    match query {
        Ok(Some(query)) => {
            let url: String = query.get("url");
            Ok(Redirect::permanent(&url[..]))
        }
        _ => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

pub struct App {
    state: AppState,
}

impl App {
    pub async fn create(db: PgPool) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            state: AppState { db },
        })
    }

    pub async fn router(&self) -> Result<Router, Box<dyn std::error::Error>> {
        let session_store = PostgresStore::new(self.state.db.clone());
        session_store.migrate().await?;

        let deletion_task = tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );

        let session_layer = SessionManagerLayer::new(session_store).with_secure(false); // @TODO: with_secure(true)
        // .with_expiry(Expiry::OnInactivity(::from_secs(10))); // @TODO

        let backend = Backend::new(self.state.db.clone());
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let router = Router::new()
            .route("/", post(shorten))
            .route_layer(auth_layer)
            .route("/{path}", get(redirect))
            .layer(
                ServiceBuilder::new().layer(
                    CorsLayer::new()
                        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                        .allow_methods(Any)
                        .allow_origin(
                            std::env::var("ALLOW_ORIGIN")
                                .unwrap()
                                .parse::<HeaderValue>()?,
                        ),
                ),
            )
            .with_state(self.state.clone());

        Ok(router)
    }
}
