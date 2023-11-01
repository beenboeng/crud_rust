use actix_web::{http::StatusCode, web, HttpResponse, Responder, Result};
use diesel::{
    sql_query,
    sql_types::{Bool, Integer, Text},
};

use crate::{
    error::ServiceError,
    models::user_model::{UserRequest, UserRespone},
};

use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

pub async fn show(pool: web::Data<Pool<AsyncPgConnection>>) -> Result<HttpResponse> {
    let mut conn = pool.get().await.unwrap();

    let raw_query = "
    SELECT u.id,first_name, u.last_name, u.username, u.email, u.phone, g.lookup_name as gender, r.lookup_name as role
    FROM tbl_users u
    INNER JOIN tbl_lookup g ON u.gender_id = g.lookup_id AND g.type = 'gender'
    INNER JOIN tbl_lookup r ON u.role_id = r.lookup_id AND r.type = 'role';
    ";

    let results: Result<Vec<UserRespone>, ServiceError> = sql_query(raw_query)
        .get_results(&mut conn)
        .await
        .map_err(|_| ServiceError::new(StatusCode::BAD_REQUEST, false, "Error".to_string(), -1));

    println!("{:#?}", results);
    match results {
        Ok(rs) => Ok(HttpResponse::Ok().json(rs)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

// pub async fn add(
//     user_request: web::Json<UserRequest>,
//     pool: web::Data<Pool<AsyncPgConnection>>,
// ) -> Result<HttpResponse> {
//     let mut conn = pool.get().await.unwrap();

//     let raw_query = "INSERT INTO tbl_todo(
//         title, completed, task_status, percent)
//        VALUES ( $1, $2, $3, $4);";

//     let results: Result<Vec<UserRequest>, ServiceError> = sql_query(raw_query)
//         .bind::<Text, _>(user_request.0.title)
//         .bind::<Bool, _>(user_request.0.completed)
//         .bind::<Text, _>(user_request.0.task_status)
//         .bind::<Integer, _>(user_request.0.percent)
//         // .bind::<Integer, _>(user_request.0.percent.unwrap_or(""))
//         .get_results(&mut conn)
//         .await
//         .map_err(|_| ServiceError::new(StatusCode::BAD_REQUEST, false, "Error".to_string(), -1));

//     match results {
//         Ok(_) => Ok(HttpResponse::Ok().body("Added Done")),
//         Err(_) => Ok(HttpResponse::Ok().body("Error")),
//     }
// }
