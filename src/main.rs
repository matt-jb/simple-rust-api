use actix_web::{App, HttpServer, middleware::Logger};
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(api::get_fuel_usage)
            .service(api::get_fail_probability)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}