use actix_web::{http::StatusCode, web, HttpResponse, Result};
use diesel::{sql_query, sql_types::Bool};

use crate::{
    error::ServiceError,
    models::task_model::{TaskRequest, TaskRespone},
};

use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

pub async fn show(pool: web::Data<Pool<AsyncPgConnection>>) -> Result<HttpResponse> {
    let mut conn = pool.get().await.unwrap();

    let raw_query = "SELECT * FROM tbl_todo";

    let results: Result<Vec<TaskRespone>, ServiceError> = sql_query(raw_query)
        // .bind::<Integer, _>(found_user.id)
        .get_results(&mut conn)
        .await
        .map_err(|_| ServiceError::new(StatusCode::BAD_REQUEST, false, "Error".to_string(), -1));

    match results {
        Ok(rs) => Ok(HttpResponse::Ok().json(rs)),
        Err(_) => Ok(HttpResponse::Ok().body("Error")),
    }
}

pub async fn add(
    task_request: web::Json<TaskRequest>,
    pool: web::Data<Pool<AsyncPgConnection>>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().await.unwrap();

    let raw_query = "INSERT INTO tbl_todo(
        title, completed, task_status, percent)
       VALUES ( $1, $2, $3, $4);";

    let results: Result<Vec<TaskRespone>, ServiceError> = sql_query(raw_query)
        // .bind::<String, _>(task_request.0.title)
        // .bind::<Bool, _>(task_request.0.completed)
        // .bind::<String, _>(task_request.0.task_status)
        // .bind::<String, _>(task_request.0.percent)
        .get_results(&mut conn)
        .await
        .map_err(|_| ServiceError::new(StatusCode::BAD_REQUEST, false, "Error".to_string(), -1));

    match results {
        Ok(_) => Ok(HttpResponse::Ok().body("Added Done")),
        Err(_) => Ok(HttpResponse::Ok().body("Error")),
    }
}

// pub async fn edit(task_update: web::Json<TaskEdit>, task_id: web::Path<(u32,)>) -> impl Responder {
//     let id = task_id.0;
//     println!("id: {}", id);
//     HttpResponse::Ok().json(task_update)
// }

// pub async fn delete(task_id: web::Path<(u32,)>) -> impl Responder {
//     let id = task_id.0;
//     println!("id: {}", id);
//     HttpResponse::Ok().body("deleted")
// }
