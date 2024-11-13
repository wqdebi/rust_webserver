use std::io::Empty;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;
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
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData{
    email: String,
    name: String,
}
async fn subscribe(_form: web::Form<FormData>)->HttpResponse {
    HttpResponse::Ok().finish()
}


pub fn run(linstener: TcpListener)->Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/{name}", web::get().to(greet))
            .route("/{name}/{a}/{b}", web::get().to(add))
    })
        .listen(linstener)?
        .run();
    Ok(server)
}