use entity::user;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserDto {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: String,
}

impl user::IntoActiveModel for UserDto {
    fn into_active_model(self) -> user::ActiveModel {
        let now = chrono::offset::Utc::now();
        user::ActiveModel {
            phone_number: Set(self.phone_number.to_owned()),
            first_name: Set(self.first_name.to_owned()),
            last_name: Set(self.last_name.to_owned()),
            datetime_utc: Set(now.naive_utc()),
            ..Default::default()
        }
    }
}
