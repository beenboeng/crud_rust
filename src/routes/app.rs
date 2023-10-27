use actix_web::web::{self};


mod tasks {
    use crate::api::tasks;
    use actix_web::web;

    pub fn tasks_routes(tasks_config: &mut web::ServiceConfig) {
        tasks_config.service(
            web::scope("")
                .route("/list", web::get().to(tasks::show))
                .route("/add", web::post().to(tasks::add))
                .route("/edit/{id}", web::post().to(tasks::edit))
                .route("/delete/{id}", web::delete().to(tasks::delete))
                
        );
    }
}

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/web")
            .configure(tasks::tasks_routes)
            
    );
}
