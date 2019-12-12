mod auth;
mod endpoints;
mod error;
mod models;
mod responses;
mod sync;
mod toggl_api;

fn main() -> std::io::Result<()> {
    use actix_web::{
        middleware::{Compress, Logger},
        web, App, HttpServer,
    };

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let addr = "localhost:8080";
    println!("Starting the server at {}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(Compress::default())
            .service(web::resource("/current-snapshot").route(web::get().to(endpoints::login)))
            .service(web::resource("/sync").route(web::post().to(endpoints::sync)))
    })
    .bind(addr)?
    .run()
}
