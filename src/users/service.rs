use super::dto::UserDto;
use crate::error::MyError;
use entity::user::{self, IntoActiveModel};
use sea_orm::{
    entity::ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait,
    QueryFilter, Set,
};

pub async fn add_user(db: &DatabaseConnection, user_info: UserDto) -> Result<user::Model, MyError> {
    let user = user_info.into_active_model();

    let user = user.insert(db).await?;

    Ok(user)
}

pub async fn get_user_by_id(db: &DatabaseConnection, id: i32) -> Result<user::Model, MyError> {
    user::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(MyError::DbErr)?
        .ok_or("User not found!".into())
        .map_err(MyError::NotFound)
}

pub async fn update_user(
    db: &DatabaseConnection,
    id: i32,
    user_info: UserDto,
) -> Result<user::Model, MyError> {
    let mut user = user_info.into_active_model();
    user.id = Set(id);

    // let mut user: user::ActiveModel = user.unwrap().into();
    // let now = chrono::offset::Utc::now();
    // user.id = Set(id);
    // user.first_name = Set(user_info.first_name);
    // user.last_name = Set(user_info.last_name);
    // user.datetime_utc = Set(now.naive_utc());

    let user = user.update(db).await?;

    Ok(user)
}

pub async fn delete_user(db: &DatabaseConnection, id: i32) -> Result<bool, MyError> {
    let user = get_user_by_id(db, id).await?;
    let res = user.delete(db).await?;
    Ok(res.rows_affected > 0)
}

pub async fn _get_user_by_phone_number(
    db: &DatabaseConnection,
    phone_number: &str,
) -> Result<user::Model, MyError> {
    user::Entity::find()
        .filter(user::Column::PhoneNumber.eq(phone_number))
        .one(db)
        .await
        .map_err(MyError::DbErr)?
        .ok_or("User not found!".into())
        .map_err(MyError::NotFound)
}

// #[cfg(test)]
// // #[cfg(feature = "seaorm-mock")]
// mod tests {
//     use super::*;
//     use entity::user;
//     use sea_orm::{
//         tests_cfg::*, ConnectionTrait, DatabaseConnection, DbBackend, MockDatabase, Statement,
//         Transaction,
//     };
//     // use futures::TryStreamExt;
//     use chrono::NaiveDate;
//     // use once_cell::sync::Lazy;
//     // use sea_orm::sea_query::{Alias, Expr, SelectStatement, Value};

//     // static RAW_STMT: Lazy<Statement> = Lazy::new(|| {
//     //     Statement::from_sql_and_values(
//     //         DbBackend::Postgres,
//     //         r#"SELECT "fruit"."id", "fruit"."name", "fruit"."cake_id" FROM "fruit""#,
//     //         vec![],
//     //     )
//     // });

//     fn setup() -> (DatabaseConnection, Vec<Vec<user::Model>>) {
//         let page1 = vec![
//             user::Model {
//                 id: 1,
//                 phone_number: "0912333457".into(),
//                 first_name: "milad".into(),
//                 last_name: "rezaee".into(),
//                 datetime_utc: NaiveDate::from_ymd(2020, 7, 8).and_hms(8, 10, 11),
//             },
//             user::Model {
//                 id: 2,
//                 phone_number: "09173255431".into(),
//                 first_name: "ehsan".into(),
//                 last_name: "amiri".into(),
//                 datetime_utc: NaiveDate::from_ymd(2020, 8, 10).and_hms(9, 11, 9),
//             },
//         ];

//         let page2 = vec![user::Model {
//             id: 3,
//             phone_number: "09374565431".into(),
//             first_name: "ahmad".into(),
//             last_name: "akbari".into(),
//             datetime_utc: NaiveDate::from_ymd(2020, 9, 10).and_hms(9, 11, 9),
//         }];

//         let page3 = Vec::<user::Model>::new();

//         let db = MockDatabase::new(DbBackend::Postgres)
//             .append_query_results(vec![page1.clone(), page2.clone(), page3.clone()])
//             .into_connection();

//         (db, vec![page1, page2, page3])
//     }

//     #[actix_web::test]
//     async fn test_get_user_by_phone_number() {
//         let (db, pages) = setup();
//         // assert_eq!(
//         //     get_user_by_phone_number(&db, &pages[0][0].phone_number)
//         //         .await
//         //         .unwrap(),
//         //     pages[0][0].clone()
//         // );
//         // assert_eq!(
//         //     get_user_by_phone_number(&db, "09173255431").await.unwrap(),
//         //     pages[0][1].clone()
//         // );
//         assert_eq!(
//             get_user_by_phone_number(&db, &pages[1][0].phone_number)
//                 .await
//                 .unwrap(),
//             pages[1][0].clone()
//         );
//         // assert_eq!(
//         //     user::Entity::find()
//         //         .filter(user::Column::PhoneNumber.contains("0912333457"))
//         //         .one(&db)
//         //         .await
//         //         .unwrap(),
//         //     pages[1][0].clone()
//         // );

//         // assert_eq!(
//         //     user::Entity::find()
//         //         .from_raw_sql(Statement::from_sql_and_values(
//         //             DbBackend::Postgres,
//         //             r#"SELECT * FROM "users2" WHERE "phone_number" = $1"#,
//         //             // r#"SELECT "cake"."id", "cake"."name" FROM "cake" WHERE "id" = $1"#,
//         //             // vec![phone_number.into()],
//         //             vec!["0912333457".into()],
//         //         ))
//         //         .one(&db)
//         //         .await
//         //         .unwrap(),
//         //     pages[1][0].clone()
//         // );
//     }
// }
