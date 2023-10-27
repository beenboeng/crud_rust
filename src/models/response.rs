use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub success: bool,     //The status is True/False  resonse from server api
    pub message: String,   //System description both success or failed
    pub status_code: i64,  //The NUMBER CODE standard implement on future the purpose API connect busines and business
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(success: bool, message: &str, status_code: i64, data: T) -> ResponseBody<T> {
        ResponseBody {
            success,
            message: message.to_string(),
            status_code,
            data,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link{
    pub first: Option<String>,//"/users?page=1&limit=10"
    pub last:  Option<String>,//"/users?page=10&limit=10",
    pub prev:  Option<String>,//"/users?page=1&limit=10",
    pub next:  Option<String>,//"/users?page=3&limit=10"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBodyPaging<T> {
    pub success: bool,
    pub message: String,
    pub status_code: i64,
    pub data: T,
    pub page: i64,
    pub limit : i64,
    pub total: i64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,

}

impl<T> ResponseBodyPaging<T> {
    #[allow(dead_code)]
    pub fn new(
        success: bool,
        message: &str,
        status_code: i64,
        data: T,

        page: i64,
        limit: i64,
        total: i64,
        links: Option<Vec<Link>>,
    ) -> ResponseBodyPaging<T> {
        ResponseBodyPaging {
            success,
            message: message.to_string(),
            status_code,
            data,
            page,
            limit,
            total,
            links,
        }
    }
}
