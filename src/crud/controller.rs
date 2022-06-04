use super::{
    dto::{Contacts, User},
    service,
};
use crate::error::MyError;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

#[post("/")]
pub async fn create_user(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_user = service::create_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}

#[delete("/")]
pub async fn delete_user(
    phone_number: String,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let deleted_user = service::delete_user(&client, &phone_number).await?;

    Ok(HttpResponse::Ok().json(deleted_user))
}

#[put("/")]
pub async fn update_user(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let updated_user = service::update_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(updated_user))
}

#[get("/{phone_number}")]
pub async fn get_user(
    phone_number: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let user = service::get_user(&client, &phone_number.into_inner()).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/birthdays")]
pub async fn get_birthdays(
    contacts: web::Json<Contacts>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let birthdays = service::get_birthdays(&client, &contacts.members).await?;

    Ok(HttpResponse::Ok().json(birthdays))
}
