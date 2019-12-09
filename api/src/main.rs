mod auth;
mod endpoints;
mod models;
mod responses;
mod sync;
mod toggl_api;

fn main() -> std::io::Result<()> {
    use actix_web::{middleware, web, App, HttpServer};

    std::env::set_var("RUST_LOG", "actix_web=info");

    let addr = "localhost:8080";
    println!("Starting the server at {}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(web::resource("/login").route(web::post().to(endpoints::login)))
            .service(web::resource("/sync").route(web::post().to(endpoints::login)))
    })
    .bind(addr)?
    .run()
}
