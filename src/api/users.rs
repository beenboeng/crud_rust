use actix_web::{http::StatusCode, web, HttpResponse, Result, Responder};
use diesel::{
    sql_query,
    sql_types::{Bool, Integer, Text},
};

use crate::{
    error::ServiceError,
    models::task_model::{TaskRequest, TaskRespone},
};

use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};




pub async fn show(pool: web::Data<Pool<AsyncPgConnection>>) ->  impl Responder {
    // let mut conn = pool.get().await.unwrap();

    // let raw_query = "SELECT * FROM tbl_todo";

    // let results: Result<Vec<TaskRespone>, ServiceError> = sql_query(raw_query)
    //     .get_results(&mut conn)
    //     .await
    //     .map_err(|_| ServiceError::new(StatusCode::BAD_REQUEST, false, "Error".to_string(), -1));

    // match results {
    //     Ok(rs) => Ok(HttpResponse::Ok().json(rs)),
    //     Err(_) => Ok(HttpResponse::Ok().body("Error")),
    // }

    HttpResponse::Ok().body("users list")
}