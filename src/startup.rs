pub mod routes;
pub fn run(linstener: TcpListener) ->Result<Server, std::io::Error> {
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