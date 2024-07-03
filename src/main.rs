use actix_web::{App, HttpServer, web};

mod api;
mod revproxy;
mod config;

use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env().expect("Server configuration");
    let config = web::Data::new(config);

    println!("Starting server in {} mode", if config.is_development() { "DEVELOPMENT" } else { "PRODUCTION" });
    println!("Development frontend port: {}", config.devfrontport);

    HttpServer::new(move || {
        let app = App::new().app_data(config.clone());

        if config.is_development() {
            app.configure(revproxy::routes::dev_config)
        } else {
            app.configure(revproxy::routes::prod_config)
        }
    })
    .bind(("127.0.0.1", 2080))?
    .run()
    .await
}