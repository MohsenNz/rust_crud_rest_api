mod controller;
mod dto;
mod service;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(controller::add_user)
            .service(controller::get_user)
            .service(controller::update_user)
            .service(controller::delete_user),
    );
}
