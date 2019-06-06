use actix_web::{Error, HttpResponse};
use futures::Future;

pub trait FutureResponse: Future<Item = HttpResponse, Error = Error> {}
impl<T: Future<Item = HttpResponse, Error = Error>> FutureResponse for T {}
