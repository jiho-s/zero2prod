use actix_web::{App, guard, HttpRequest, HttpServer, Responder, Route, web};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", Route::new().guard(guard::Get()).to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
