use futures_util::{FutureExt as _, Stream};
use hyper::body::HttpBody as _;

pub struct ProxyDownloadStream {
    response: hyper::Response<hyper::Body>,
}

impl ProxyDownloadStream {
    pub fn new(body: hyper::Response<hyper::Body>) -> Self {
        Self { response: body }
    }
}

impl Stream for ProxyDownloadStream {
    type Item = Result<actix_web::web::Bytes, actix_web::Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match self.response.body_mut().data().poll_unpin(cx) {
            std::task::Poll::Ready(result) => match result {
                Some(result) => match result {
                    Ok(result) => std::task::Poll::Ready(Some(Ok(result))),
                    Err(error) => todo!("{:?}", error),
                },
                None => std::task::Poll::Ready(None),
            },
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}
