use actix_web::{web, Error, Responder};
use bson::oid::ObjectId;
use bson::{bson, doc};
use mongodb::options::FindOptions;
use mongodb::Client;

use crate::models::class::{Class, ClassDetailRequest, ClassDetailResponse, ClassListResponse};
use crate::utils::mongo::{fetch_items, get_coll};

pub async fn class_detail_handle(
    info: web::Query<ClassDetailRequest>,
    client: web::Data<Client>,
) -> Result<impl Responder, Error> {
    let id = info.id.clone();
    let name = info.name.clone();
    let mut filter = doc! {};
    if let Some(name) = name {
        filter = doc! {"name": name};
    }
    if let Some(id) = id {
        filter = doc! {"_id": ObjectId::with_string(id.as_str()).unwrap()};
    }
    let coll = get_coll(&client, "lianxibk", "class");
    let document = coll.find_one(filter, None).unwrap().unwrap();
    let item = Class::from(document);

    Ok(ClassDetailResponse {
        status: 200,
        message: "ok".into(),
        data: item,
    })
}

pub async fn class_list_handle(client: web::Data<Client>) -> Result<impl Responder, Error> {
    let coll = get_coll(&client, "lianxibk", "class");
    let find_options = FindOptions::builder()
        .sort(doc! {"recommend_status": -1, "created_time": -1})
        .batch_size(10)
        .limit(10)
        .build();
    let cursor = coll.find(doc! {}, find_options).unwrap();
    let result = fetch_items::<Class>(cursor);

    Ok(ClassListResponse {
        status: 200,
        message: "ok".into(),
        data: result,
    })
}
