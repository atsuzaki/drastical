extern crate actix_web;
use actix_web::{server, App, HttpRequest, HttpResponse, http, Json, Responder};
use actix_web::{middleware};

fn index(req: &HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Request received\n")
}

fn main() {
    println!("Starting http server");
    let sys = actix::System::new("digidailies-service");

    server::new(
        || App::new()
            .middleware(middleware::Logger::default())
            .resource("/push", |r| r.method(http::Method::POST).f(index))
        )

        .bind("127.0.0.1:8088")
        .unwrap()
        .start();

    println!("Server running on 127.0.0.1:8088");
    let _ = sys.run();
}
