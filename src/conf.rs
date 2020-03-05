use actix_web::web;

use crate::api::*;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.route("/heartbeat", web::get().to(heartbeat::heartbeat_handle))
        .service(
            web::scope("/api/")
                .service(
                    web::scope("/banner")
                        .route("/list", web::get().to(banner::banner_list_handle))
                        .route("/detail", web::get().to(banner::banner_detail_handle)),
                )
                .service(
                    web::scope("/entry")
                        .route("/list", web::get().to(entry::entry_list_handle))
                        .route("/detail", web::get().to(entry::entry_detail_handle)),
                )
                .service(
                    web::scope("/class")
                        .route("/list", web::get().to(class::class_list_handle))
                        .route("/detail", web::get().to(class::class_detail_handle)),
                ),
        );
}
