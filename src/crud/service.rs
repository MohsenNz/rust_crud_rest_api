use super::dto::{Birthday, User};
use crate::error::MyError;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn create_user(client: &Client, user_info: User) -> Result<User, MyError> {
    let _stmt = include_str!("../../sql/create_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &user_info.phone_number,
                &user_info.first_name,
                &user_info.last_name,
                &user_info.birthday,
                &user_info.contacts,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}

pub async fn get_user(client: &Client, phone_number: &str) -> Result<User, MyError> {
    let _stmt = include_str!("../../sql/get_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&phone_number])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn update_user(client: &Client, user_info: User) -> Result<User, MyError> {
    let _stmt = include_str!("../../sql/update_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &user_info.phone_number,
                &user_info.first_name,
                &user_info.last_name,
                &user_info.birthday,
                &user_info.contacts,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}

pub async fn delete_user(client: &Client, phone_number: &str) -> Result<User, MyError> {
    let _stmt = include_str!("../../sql/delete_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&phone_number])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn get_birthdays(
    client: &Client,
    contacts: &Vec<String>,
) -> Result<Vec<Birthday>, MyError> {
    let _stmt = include_str!("../../sql/get_userbirthday.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    let mut res = vec![];

    for num in contacts {
        let brday = client
            .query(&stmt, &[&num])
            .await?
            .iter()
            .map(|row| Birthday::from_row_ref(row).unwrap())
            .collect::<Vec<Birthday>>()
            .pop();

        if let Some(x) = brday {
            res.push(x);
        }
    }

    Ok(res)
}
