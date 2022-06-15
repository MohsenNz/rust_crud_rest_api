use super::{dto::UserDto, service};
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;

#[post("/")]
pub async fn add_user(
    user: web::Json<UserDto>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let new_user = service::add_user(&db, user.clone()).await?;

    Ok(HttpResponse::Ok().json(new_user))
}

#[put("/{id}")]
pub async fn update_user(
    id: web::Path<i32>,
    user: web::Json<UserDto>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let user = user.into_inner();
    let updated_user = service::update_user(&db, *id, user).await?;

    Ok(HttpResponse::Ok().json(updated_user))
}

#[get("/{id}")]
pub async fn get_user(
    id: web::Path<i32>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let user = service::get_user_by_id(&db, *id).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/{id}")]
pub async fn delete_user(
    id: web::Path<i32>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let _ = service::delete_user(&db, *id).await?;

    Ok(HttpResponse::Ok().finish())
}
