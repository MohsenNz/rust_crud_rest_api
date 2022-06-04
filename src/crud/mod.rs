mod controller;
mod dto;
mod service;

use actix_web::{web, Scope};

pub fn services() -> Scope {
    web::scope("/users")
        .service(controller::create_user)
        .service(controller::get_user)
        .service(controller::update_user)
        .service(controller::delete_user)
        .service(controller::get_birthdays)
}
