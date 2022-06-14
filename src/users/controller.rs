use super::{dto::UserDto, service};
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;

#[post("/")]
pub async fn add_user(
    user: web::Json<UserDto>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let _new_user_id = service::add_user(&db, user.clone()).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/")]
pub async fn update_user(
    user: web::Json<UserDto>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let user = user.into_inner();
    let updated_user_phone = service::update_user(&db, user).await?;

    Ok(HttpResponse::Ok().json(updated_user_phone))
}

#[get("/{phone_number}")]
pub async fn get_user(
    phone_number: web::Path<String>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let user = service::get_user_by_phone_number(&db, &phone_number).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/{phone_number}")]
pub async fn delete_user(
    phone_number: web::Path<String>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let is_deleted = service::delete_user(&db, &phone_number).await?;

    Ok(HttpResponse::Ok().json(is_deleted))
}
