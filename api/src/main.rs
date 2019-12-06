use actix_web::{middleware, web, App, HttpServer};

mod endpoints;
mod models;
mod responses;
mod sync;
mod toggl_api;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    let addr = "localhost:8080";

    println!("Starting the server at {}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            // .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/login").route(web::post().to(endpoints::login)))
            .service(web::resource("/sync").route(web::post().to(endpoints::login)))
    })
    .bind(addr)?
    .run()
}
