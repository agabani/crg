use actix_web::{web, HttpRequest, HttpResponse};

use crate::proxy;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("{path:.*}", web::to(proxy));
}

#[allow(clippy::unused_async)]
async fn proxy(request: HttpRequest, body: web::Payload) -> actix_web::Result<HttpResponse> {
    let backend = proxy::Backend::new("https://registry-1.docker.io".to_string());

    let response = backend.forward(request, body).await;

    Ok(response)
}
