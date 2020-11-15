use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    let city = req.match_info().get("city").unwrap_or("");
    if city != "" {
        format!("Hello {}! from: {}\n", &name, &city)
    } else {
        format!("Hello {}!\n", &name)
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/{name}/{city}", web::get().to(greet))
            .route("/health-check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
