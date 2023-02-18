use crate::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub cant_migrate_condition: Value,
    pub create_time: i64,
    pub end_time: i64,
    pub freeze_album: bool,
    pub id: i64,
    pub item_count: i64,
    pub name: String,
    pub owner_user_id: i64,
    pub passphrase: String,
    pub shared: bool,
    pub sort_by: String,
    pub sort_direction: String,
    pub start_time: i64,
    pub temporary_shared: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResponse {
    pub album: Album,
    pub error_list: Vec<Value>,
}

#[derive(Debug, Clone, Default)]
pub struct CreateRequest {
    pub name: String,
    pub items: Vec<i64>,
}

impl Request for CreateRequest {
    type Response = CreateResponse;
    fn query(&self) -> String {
        let Self {name, items} = self;
        let name = urlencoding::encode(name);
        format!("api=SYNO.Foto.Browse.NormalAlbum&method=create&version=1&name={name}&item={items:?}")
    }
}

#[test]
fn test_parse_create_response() {
    let response = include_str!("test/create-album-response.json");
    let parsed: Response<CreateResponse> = serde_json::from_str(response).unwrap();
    let data = parsed.body.as_result().unwrap();
    assert_eq!("normal".to_owned(), data.album.type_field);
    assert!(data.error_list.is_empty());
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponse {
    pub list: Vec<Album>,
}

#[derive(Default, Debug, Clone)]
pub struct ListRequest {
    pub offset: u32,
    pub limit: u32,
    pub sort_by: String,
    pub sort_direction: String,
}

impl Request for ListRequest {
    type Response = ListResponse;
    fn query(&self) -> String {
        let Self { offset, limit, sort_by, sort_direction } = self;
        format!("api=SYNO.Foto.Browse.Album&method=list&version=2&offset={offset}&limit={limit}&sort_by={sort_by}&sort_direction={sort_direction}")
    }
}

#[test]
fn test_parse_album_list_response() {
    let response = include_str!("test/album-list-response.json");
    let parsed: Response<ListResponse> = serde_json::from_str(response).unwrap();
    assert!(parsed.body.as_result().is_ok());
}