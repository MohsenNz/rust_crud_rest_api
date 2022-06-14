use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use sea_orm::DbErr;

#[derive(Display, From, Debug)]
pub enum MyError {
    NotFound(String),
    DbErr(DbErr),
}

impl std::error::Error for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::NotFound(err) => {
                log::info!("NotFound: {}", err);
                // HttpResponse::Accepted().finish()
                HttpResponse::NotFound().finish()
            }
            MyError::DbErr(err) => {
                log::error!("DbErr: {}", err.to_string());
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
