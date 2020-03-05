use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use bson::ordered::OrderedDocument;
use bson::Document;
use futures::future::{ready, Ready};

#[derive(Debug, Serialize)]
pub struct Class {
    pub id: String,
    pub name: String,
    pub recommend_status: isize,
    pub created_time: String,
}

impl From<Document> for Class {
    fn from(doc: OrderedDocument) -> Self {
        Class {
            id: doc.get_object_id("_id").unwrap().to_hex(),
            name: doc.get_str("name").unwrap().to_string(),
            recommend_status: doc.get_i32("recommend_status").unwrap() as isize,
            created_time: doc.get_str("created_time").unwrap().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ClassDetailRequest {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ClassDetailResponse {
    pub status: u16,
    pub message: String,
    pub data: Class,
}

impl Responder for ClassDetailResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[derive(Debug, Serialize)]
pub struct ClassListResponse {
    pub status: u16,
    pub message: String,
    pub data: Vec<Class>,
}

impl Responder for ClassListResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
