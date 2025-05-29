use std::pin::Pin;

use axum::http::{Request, Response};
use tower_http::auth::AsyncAuthorizeRequest;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

#[derive(Clone)]
pub struct TokenAuth {}

impl<B: 'static> AsyncAuthorizeRequest<B> for TokenAuth {
    type RequestBody = B;
    type ResponseBody = axum::body::Body;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, request: Request<B>) -> Self::Future {
        Box::pin(async { Ok(request) })
    }
}
