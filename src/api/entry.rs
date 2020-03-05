use actix_web::{web, Error, Responder};
use bson::oid::ObjectId;
use bson::{bson, doc};
use mongodb::options::FindOptions;
use mongodb::Client;

use crate::models::entry::{Entry, EntryDetailRequest, EntryDetailResponse, EntryListResponse};
use crate::utils::mongo::{fetch_items, get_coll};

pub async fn entry_detail_handle(
    info: web::Query<EntryDetailRequest>,
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
    let coll = get_coll(&client, "lianxibk", "entry");
    let document = coll.find_one(filter, None).unwrap().unwrap();
    let item = Entry::from(document);

    Ok(EntryDetailResponse {
        status: 200,
        message: "ok".into(),
        data: item,
    })
}

pub async fn entry_list_handle(client: web::Data<Client>) -> Result<impl Responder, Error> {
    let coll = get_coll(&client, "lianxibk", "entry");
    let find_options = FindOptions::builder()
        .sort(doc! {"show_index": 1})
        .batch_size(10)
        .limit(10)
        .build();
    let cursor = coll.find(doc! {}, find_options).unwrap();
    let result = fetch_items::<Entry>(cursor);

    Ok(EntryListResponse {
        status: 200,
        message: "ok".into(),
        data: result,
    })
}
