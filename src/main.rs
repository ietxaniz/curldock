use actix_web::{App, HttpServer};

mod api;
mod revproxy;
mod config;
mod script_manager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::initialize_config();
    script_manager::initialize_script_manager();

    let config = config::get_config();

    println!("Starting server in {} mode", if config.is_development() { "DEVELOPMENT" } else { "PRODUCTION" });
    println!("Development frontend port: {}", config.devfrontport);

    HttpServer::new(|| {
        let app = App::new();

        if config::get_config().is_development() {
            app.configure(revproxy::routes::dev_config)
        } else {
            app.configure(revproxy::routes::prod_config)
        }
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}