use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use bson::ordered::OrderedDocument;
use bson::Document;
use futures::future::{ready, Ready};

#[derive(Debug, Serialize)]
pub struct Banner {
    pub id: String,
    pub name: String,
    pub img_url: String,
    pub class_id: String,
    pub show_index: isize,
    pub created_time: String,
}

impl From<Document> for Banner {
    fn from(doc: OrderedDocument) -> Self {
        Banner {
            id: doc.get_object_id("_id").unwrap().to_hex(),
            name: doc.get_str("name").unwrap().to_string(),
            img_url: doc.get_str("img_url").unwrap().to_string(),
            class_id: doc.get_str("class_id").unwrap().to_string(),
            show_index: doc.get_i32("show_index").unwrap() as isize,
            created_time: doc.get_str("created_time").unwrap().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BannerDetailRequest {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BannerDetailResponse {
    pub status: u16,
    pub message: String,
    pub data: Banner,
}

impl Responder for BannerDetailResponse {
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
pub struct BannerListResponse {
    pub status: u16,
    pub message: String,
    pub data: Vec<Banner>,
}

impl Responder for BannerListResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
