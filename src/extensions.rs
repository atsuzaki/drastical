use actix_web::{FutureResponse, HttpResponse};
use futures::future::ok;

pub trait BoxResponse {
    fn boxed_future(self) -> FutureResponse<HttpResponse>;
}

impl BoxResponse for HttpResponse {
    fn boxed_future(self) -> FutureResponse<HttpResponse> {
        Box::new(ok(self))
    }
}
