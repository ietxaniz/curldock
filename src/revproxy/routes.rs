use super::handlers::{api, dev_proxy, prod_proxy, test};
use crate::config::Config;
use actix_web::web;

pub fn dev_config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::<Config>::clone).service(
        web::scope("")
            .service(web::scope("/api").default_service(web::route().to(api::handle_api)))
            .service(web::scope("/test").default_service(web::route().to(test::handle_test)))
            .default_service(web::route().to(dev_proxy::handle_dev_proxy)),
    );
}

pub fn prod_config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::<Config>::clone).service(
        web::scope("")
            .service(web::scope("/api").default_service(web::route().to(api::handle_api)))
            .service(web::scope("/test").default_service(web::route().to(test::handle_test)))
            .default_service(web::route().to(prod_proxy::handle_prod_proxy)),
    );
}
