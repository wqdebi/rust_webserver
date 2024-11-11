use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
async fn greet(req: HttpRequest)->impl  Responder{
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
async fn add(req: HttpRequest)->impl  Responder{
    let a = req.match_info().get("a").unwrap_or("0").parse::<i32>().unwrap_or(0);
    let b = req.match_info().get("b").unwrap_or("0").parse::<i32>().unwrap_or(0);
    format!("Hello {}!", a+b)
}

async fn health_check(_req: HttpRequest)->impl  Responder{
    HttpResponse::Ok()
}
#[tokio::main]//过程宏
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
            .route("/{name}/{a}/{b}", web::get().to(add))
    }).bind("127.0.0.1:8080")?
        .run()
        .await
}
