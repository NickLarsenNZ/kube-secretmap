use actix_web::{HttpServer, App, middleware, dev::Server};

mod endpoints {
    use actix_web::{HttpRequest, Responder, HttpResponse, get};

    // #[get("/something")]
    // async fn something(_: HttpRequest) -> impl Responder {
    //     HttpResponse::Ok().json(String::from("Hello"))
    // }

    #[get("/")]
    async fn index(_: HttpRequest) -> impl Responder {
        HttpResponse::Ok().body("might show some basic state info here")
    }

    #[get("/health")]
    async fn health(_: HttpRequest) -> impl Responder {
        HttpResponse::Ok().json("healthy")
    }


}

pub fn run(bind_addr: &str) -> Server {

    HttpServer::new(move || {
        App::new()
            //.app_data(metrics_data)
            .wrap(middleware::Logger::default().exclude("/health"))
            .service(endpoints::index)
            .service(endpoints::health)

    })
    .bind(bind_addr)
    .unwrap_or_else(|_| panic!("cannot bind to {}", bind_addr))
    .shutdown_timeout(5)
    .run()

}
