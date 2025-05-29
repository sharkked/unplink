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
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use shuttle_runtime::SecretStore;
use sqlx::{PgPool, prelude::FromRow};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
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
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> JsonResult<ShortenResponse> {
    if !Url::parse(&payload.url).is_ok() {
        return Err(StatusCode::BAD_REQUEST);
    }
    // @TODO: deny own redirects
    let code = nanoid!(10, &nanoid::alphabet::SAFE); // @TODO: collision detection

    let result = sqlx::query("insert into shortlinks (url, code) values (?, ?)")
        .bind(&payload.url)
        .bind(&code)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => ShortenResponse::ok(&format!(
            "{}/{code}",
            state.secrets.get("base_url").unwrap()
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(FromRow)]
struct ShortLink {
    id: i64,
    url: String,
    code: String,
}

async fn redirect(Path(path): Path<String>, state: State<AppState>) -> HttpResult<Redirect> {
    let result: Result<Option<ShortLink>, sqlx::Error> =
        sqlx::query_as("select * from shortlinks where code = ?")
            .bind(&path)
            .fetch_optional(&state.db)
            .await;

    match result {
        Ok(Some(link)) => Ok(Redirect::permanent(&link.url[..])),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Clone)]
pub struct AppState {
    secrets: SecretStore,
    db: PgPool,
}

pub struct App {
    state: AppState,
}

impl App {
    pub async fn create(
        db: PgPool,
        secrets: SecretStore,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        sqlx::migrate!().run(&db).await?;

        Ok(Self {
            state: AppState { secrets, db },
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

        let session_layer = SessionManagerLayer::new(session_store).with_secure(false); // @TODO: change
        // .with_expiry(Expiry::OnInactivity(::from_secs(10)));

        let router = Router::new()
            .route("/", post(shorten))
            .route("/{path}", get(redirect))
            .layer(
                ServiceBuilder::new().layer(
                    CorsLayer::new()
                        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                        .allow_methods(Any)
                        .allow_origin(
                            self.state
                                .secrets
                                .get("allow_origin")
                                .unwrap()
                                .parse::<HeaderValue>()
                                .unwrap(),
                        ),
                ),
            )
            .with_state(self.state.clone());

        Ok(router)
    }
}
