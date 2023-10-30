use actix_web::web::{self};


mod tasks {
    use crate::api::{tasks};
    use actix_web::web;

    pub fn tasks_routes(tasks_config: &mut web::ServiceConfig) {
        tasks_config.service(
            web::scope("list")
                .route("", web::get().to(tasks::show))
                .route("/add", web::post().to(tasks::add))
                .route("/edit/{id}", web::post().to(tasks::edit))
                .route("/delete/{id}", web::delete().to(tasks::delete))
                
        );
    }
}

mod users {
    use crate::api::{users};
    use actix_web::web;

    pub fn users_routes(users_config: &mut web::ServiceConfig){
        users_config.service(
            web::scope("users")
            .route("", web::get().to(users::show))
        );
    }
}

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/web")
            .configure(tasks::tasks_routes)
            .configure(users::users_routes)
            
    );
}
