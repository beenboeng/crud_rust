use actix_web::{http::StatusCode, web, HttpResponse, Result};
use diesel::{
    sql_query,
    sql_types::{Bool, Integer, Text},
};

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
        .bind::<Text, _>(task_request.0.title)
        .bind::<Bool, _>(task_request.0.completed)
        .bind::<Text, _>(task_request.0.task_status)
        .bind::<Integer, _>(task_request.0.percent)
        .get_results(&mut conn)
        .await
        .map_err(|_| ServiceError::new(StatusCode::BAD_REQUEST, false, "Error".to_string(), -1));

    match results {
        Ok(_) => Ok(HttpResponse::Ok().body("Added Done")),
        Err(_) => Ok(HttpResponse::Ok().body("Error")),
    }
}

pub async fn edit(
    task_id: web::Path<i32>,
    task_request: web::Json<TaskRequest>,
    pool: web::Data<Pool<AsyncPgConnection>>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().await.unwrap();

    let raw_query = "UPDATE tbl_todo
    SET title= $1, completed= $2, task_status= $3, percent=$4
    WHERE id = $5";

    let results: Result<Vec<TaskRespone>, ServiceError> = sql_query(raw_query)
        .bind::<Text, _>(task_request.0.title)
        .bind::<Bool, _>(task_request.0.completed)
        .bind::<Text, _>(task_request.0.task_status)
        .bind::<Integer, _>(task_request.0.percent)
        .bind::<Integer, _>(task_id.into_inner())
        .get_results(&mut conn)
        .await
        .map_err(|_| ServiceError::new(StatusCode::BAD_REQUEST, false, "Error".to_string(), -1));

    match results {
        Ok(_) => Ok(HttpResponse::Ok().body("Edit Successfully")),
        Err(_) => Ok(HttpResponse::Ok().body("Error")),
    }
}

pub async fn delete(
    task_id: web::Path<i32>,
    pool: web::Data<Pool<AsyncPgConnection>>,
) -> Result<HttpResponse> {

    let mut conn = pool.get().await.unwrap();

    let raw_query = "DELETE FROM tbl_todo 
    WHERE id = $1";

    let results: Result<Vec<TaskRespone>, ServiceError> = sql_query(raw_query)
        .bind::<Integer, _>(task_id.into_inner())
        .get_results(&mut conn)
        .await
        .map_err(|_| ServiceError::new(StatusCode::BAD_REQUEST, false, "Error".to_string(), -1));

    match results {
        Ok(_) => Ok(HttpResponse::Ok().body("Delete Successfully")),
        Err(_) => Ok(HttpResponse::Ok().body("Error")),
    }
}
