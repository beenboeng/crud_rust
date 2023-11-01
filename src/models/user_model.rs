use diesel::prelude::*;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, QueryableByName, Queryable, Deserialize, Clone, PartialEq, Serialize)]
pub struct UserRespone {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub first_name: String,
    #[diesel(sql_type = Text)]
    pub last_name: String,
    #[diesel(sql_type = Text)]
    pub user_name: String,
    #[diesel(sql_type = Text)]
    pub password: String,
    #[diesel(sql_type = Text)]
    pub email: String,
    #[diesel(sql_type = Text)]
    pub phone: String,
    #[diesel(sql_type = Text)]
    pub gender: String,
    #[diesel(sql_type = Text)]
    pub role: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct UserRequest {
   pub first_name: String,
   pub last_name: String,
   pub user_name: String,
   pub password: String,
   pub email : Option<String>,
   pub phone : Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct UserEdit {
    pub title: String,
    pub completed: bool,
    pub User_status: String,
    pub percent: u32,
}
