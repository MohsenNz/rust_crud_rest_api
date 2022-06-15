mod common;

use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    http, test, web,
    web::Data,
    App,
};
use async_once::AsyncOnce;
use common::init_app_state;
use entity::user;
use lazy_static::lazy_static;
use rust_crud_restapi::users;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserDto {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: String,
}

lazy_static! {
    static ref STATE_SEED: AsyncOnce<(Data<DatabaseConnection>, Vec<user::Model>, Vec<user::Model>)> =
        AsyncOnce::new(async {
            let db = init_app_state().await;

            let page1 = vec![user::ActiveModel {
                phone_number: Set("09175142172".to_owned()),
                first_name: Set("Lucifer".to_owned()),
                last_name: Set("Morning star".to_owned()),
                datetime_utc: Set(chrono::NaiveDate::from_ymd(2021, 2, 8).and_hms(10, 1, 1)),
                ..Default::default()
            }];

            let page2 = vec![
                user::ActiveModel {
                    phone_number: Set("09183654123".to_owned()),
                    first_name: Set("Amendiel".to_owned()),
                    last_name: Set("Angle".to_owned()),
                    datetime_utc: Set(chrono::NaiveDate::from_ymd(2020, 7, 8).and_hms(8, 10, 11)),
                    ..Default::default()
                },
                user::ActiveModel {
                    phone_number: Set("09128123471".to_owned()),
                    first_name: Set("Cloey".to_owned()),
                    last_name: Set("Deker".to_owned()),
                    datetime_utc: Set(chrono::NaiveDate::from_ymd(2021, 3, 10).and_hms(2, 12, 1)),
                    ..Default::default()
                },
                user::ActiveModel {
                    phone_number: Set("09173012954".to_owned()),
                    first_name: Set("Mazakin".to_owned()),
                    last_name: Set("Evil".to_owned()),
                    datetime_utc: Set(chrono::NaiveDate::from_ymd(2019, 7, 11).and_hms(1, 1, 10)),
                    ..Default::default()
                },
            ];

            let mut immut_page = vec![];
            let mut mut_page = vec![];

            for user in page1 {
                immut_page.push(user.insert(&db).await.unwrap());
            }

            for user in page2 {
                mut_page.push(user.insert(&db).await.unwrap());
            }

            let state = web::Data::new(db);
            (state, immut_page, mut_page)
        });
}

async fn init_service(
    state: Data<DatabaseConnection>,
) -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
    test::init_service(
        App::new()
            .app_data(state)
            .service(web::scope("/api").configure(users::config)),
    )
    .await
}

macro_rules! _init_service {
    ($state:expr) => {
        test::init_service(
            App::new()
                .app_data($state)
                .service(web::scope("/api").configure(users::config)),
        )
        .await
    };
}

#[actix_web::test]
async fn get_user_returns_200_when_user_exists() {
    let (state, seed_immut, _) = STATE_SEED.get().await.clone();
    let mut app = init_service(state).await;

    for user in seed_immut {
        let req = test::TestRequest::get()
            .uri(&format!("/api/user/{}", user.id))
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}

#[actix_web::test]
async fn get_user_returns_404_when_not_found() {
    let (state, _, _) = STATE_SEED.get().await.clone();
    let mut app = init_service(state).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", 100))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn post_user_returns_200_when_user_is_valid() {
    let (state, _, _) = STATE_SEED.get().await.clone();
    let mut app = init_service(state).await;

    let user = UserDto {
        phone_number: "943822344".to_owned(),
        first_name: "fname3432".to_owned(),
        last_name: "lname3432".to_owned(),
    };

    let req = test::TestRequest::post()
        .uri("/api/user/")
        .set_json(&user)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK)
}

#[actix_web::test]
async fn post_user_returns_500_when_user_already_exists() {
    let (state, seed_immut, _) = STATE_SEED.get().await.clone();
    let mut app = init_service(state).await;

    let seed_user = &seed_immut[0];
    let user = UserDto {
        phone_number: seed_user.phone_number.clone(),
        first_name: seed_user.first_name.clone(),
        last_name: seed_user.last_name.clone(),
    };

    let req = test::TestRequest::post()
        .uri("/api/user/")
        .set_json(&user)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR)
}

#[actix_web::test]
async fn put_user_returns_200_when_user_exists() {
    let (state, _, seed_mut) = STATE_SEED.get().await.clone();
    let mut app = init_service(state).await;

    let seed_user = &seed_mut[0];
    let user = UserDto {
        phone_number: seed_user.phone_number.clone(),
        first_name: format!("{}-edited", seed_user.first_name.clone()),
        last_name: format!("{}-edited", seed_user.last_name.clone()),
    };

    let req = test::TestRequest::put()
        .uri(&format!("/api/user/{}", seed_user.id))
        .set_json(&user)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK)
}

#[actix_web::test]
async fn put_user_returns_404_when_user_does_not_exist() {
    let (state, _, _) = STATE_SEED.get().await.clone();
    let mut app = init_service(state).await;

    let user = UserDto {
        phone_number: "0".to_owned(),
        first_name: "_".to_owned(),
        last_name: "_".to_owned(),
    };

    let req = test::TestRequest::put()
        .uri(&format!("/api/user/{}", 100))
        .set_json(&user)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND)
}

#[actix_web::test]
async fn delete_user_returns_200_when_user_exists() {
    let (state, _, seed_mut) = STATE_SEED.get().await.clone();
    let mut app = init_service(state).await;

    let user = &seed_mut[2];

    let req = test::TestRequest::delete()
        .uri(&format!("/api/user/{}", &user.id))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK)
}

#[actix_web::test]
async fn delete_user_returns_404_when_user_does_not_exist() {
    let (state, _, _) = STATE_SEED.get().await.clone();
    let mut app = init_service(state).await;

    let req = test::TestRequest::delete()
        .uri(&format!("/api/user/{}", 100))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND)
}
