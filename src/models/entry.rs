use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use bson::ordered::OrderedDocument;
use bson::Document;
use futures::future::{ready, Ready};

#[derive(Debug, Serialize)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub class_id: String,
    pub show_index: isize,
    pub created_time: String,
    pub qr_code: String,
}

impl From<Document> for Entry {
    fn from(doc: OrderedDocument) -> Self {
        Entry {
            id: doc.get_object_id("_id").unwrap().to_hex(),
            name: doc.get_str("name").unwrap().to_string(),
            description: doc.get_str("description").unwrap().to_string(),
            class_id: doc.get_str("class_id").unwrap().to_string(),
            show_index: doc.get_i32("show_index").unwrap() as isize,
            created_time: doc.get_str("created_time").unwrap().to_string(),
            qr_code: doc.get_str("qr_code").unwrap().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EntryDetailRequest {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EntryDetailResponse {
    pub status: u16,
    pub message: String,
    pub data: Entry,
}

impl Responder for EntryDetailResponse {
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
pub struct EntryListResponse {
    pub status: u16,
    pub message: String,
    pub data: Vec<Entry>,
}

impl Responder for EntryListResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
