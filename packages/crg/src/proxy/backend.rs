use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use hyper::body::Sender;

use crate::proxy::proxy_download_stream::ProxyDownloadStream;

pub struct Backend {
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    host: String,
}

impl Backend {
    pub fn new(host: String) -> Self {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build();

        Self {
            client: hyper::Client::builder().build(https),
            host,
        }
    }

    pub async fn forward(
        &self,
        client_request: HttpRequest,
        mut body: web::Payload,
    ) -> HttpResponse {
        tracing::debug!(client_request = ?client_request, "client request");

        let (server_request, mut sender) = self.create_proxy_request(&client_request);
        tracing::debug!(server_request = ?server_request, "server request");

        while let Some(item) = body.next().await {
            sender.send_data(item.expect("TODO")).await.expect("TODO");
        }

        let server_response = self.client.request(server_request).await.expect("TODO");
        tracing::debug!(server_response = ?server_response, "server response");

        let client_response = Self::create_proxy_response(server_response);
        tracing::debug!(client_response = ?client_response, "client response");

        client_response
    }

    fn create_proxy_request(&self, req: &HttpRequest) -> (hyper::Request<hyper::Body>, Sender) {
        let mut builder = hyper::Request::builder()
            .method(req.method())
            .uri(&self.absolute_path(req));

        for (header_name, header_value) in req.headers() {
            if header_name.as_str().to_lowercase() != "host" {
                builder = builder.header(header_name, header_value);
            }
        }

        let (sender, body) = hyper::body::Body::channel();

        (builder.body(body).expect("TODO"), sender)
    }

    fn create_proxy_response(response: hyper::Response<hyper::Body>) -> HttpResponse {
        let mut response_builder = HttpResponse::build(response.status());

        for (header_name, header_value) in response.headers() {
            response_builder.append_header((header_name, header_value));
        }

        let stream = ProxyDownloadStream::new(response);

        response_builder.streaming(stream)
    }

    fn absolute_path(&self, req: &HttpRequest) -> String {
        format!("{}{}", self.host, req.path())
    }
}
