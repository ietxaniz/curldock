use actix_web::web;
use super::handlers::{api, dev_proxy, prod_proxy};
use crate::config::Config;

pub fn dev_config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::<Config>::clone)
        .service(
            web::scope("")
                .service(
                    web::scope("/api")
                        .default_service(web::route().to(api::handle_api))
                )
                .default_service(web::route().to(dev_proxy::handle_dev_proxy))
        );
}

pub fn prod_config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::<Config>::clone)
        .service(
            web::scope("")
                .service(
                    web::scope("/api")
                        .default_service(web::route().to(api::handle_api))
                )
                .default_service(web::route().to(prod_proxy::handle_prod_proxy))
        );
}