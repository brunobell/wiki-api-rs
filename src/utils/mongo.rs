use bson::Document;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection, Cursor};

pub fn get_client(db_url: &str, app_name: &str) -> Client {
    let mut client_options = ClientOptions::parse(db_url).unwrap();
    client_options.app_name = Some(app_name.into());
    Client::with_options(client_options).unwrap()
}

pub fn get_coll(client: &Client, db: &str, coll: &str) -> Collection {
    client.database(db).collection(coll)
}

pub fn fetch_items<T: From<Document>>(cursor: Cursor) -> Vec<T> {
    cursor
        .into_iter()
        .filter_map(|item| item.ok())
        .map(|item| T::from(item))
        .collect()
}
