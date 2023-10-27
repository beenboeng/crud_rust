use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::sql_types::*;


#[derive(Debug, QueryableByName, Queryable, Deserialize,  Clone, PartialEq, Serialize)]
pub struct TaskRespone {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub title: String,
    #[diesel(sql_type = Bool)]
    pub completed: bool,
    #[diesel(sql_type = Text)]
    pub task_status: String,
    #[diesel(sql_type = Integer)]
    pub percent: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct TaskRequest {
    pub title: String,
    pub completed: bool,
    pub task_status: String,
    pub percent: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct TaskEdit {
    pub title: String,
    pub completed: bool,
    pub task_status: String,
    pub percent: u32,
}

#[derive(Debug, QueryableByName, Queryable, Deserialize,  Clone, PartialEq, Serialize)]
pub struct Todo {
    #[diesel(sql_type = Integer)]
    pub id: i32,
}
