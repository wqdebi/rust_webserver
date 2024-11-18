use actix_web::{HttpRequest, HttpResponse, Responder};

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