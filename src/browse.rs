use crate::*;
use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponse{
    pub list: Vec<BrowseItem>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrowseItem {
    pub id: i64,
    pub filename: String,
    pub filesize: u64,
    #[serde(with = "ts_seconds")]
    pub time: DateTime,
    #[serde(with = "ts_seconds")]
    pub indexed_time: DateTime,
    pub owner_user_id: i64,
    pub folder_id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Default)]
pub struct ListRequest {
    pub offset: u32,
    pub limit: u32,
    pub start_time: DateTime,
    pub end_time: DateTime,
}

impl Request for ListRequest {
    type Response = ListResponse;
    fn query(&self) -> String {
        let Self { offset, limit, start_time, end_time } = self;
        let start_time = start_time.timestamp();
        let end_time = end_time.timestamp();
        format!("api=SYNO.Foto.Browse.Item&method=list&version=1&offset={offset}&limit={limit}&start_time={start_time}&end_time={end_time}")
    }
}


#[derive(Debug, Clone, Default)]
pub struct CommonListRequest {
    pub offset: u32,
    pub limit: u32,
    pub start_time: DateTime,
    pub end_time: DateTime,
}

impl Request for CommonListRequest {
    type Response = ListResponse;

    fn query(&self) -> String {
        let Self { offset, limit, start_time, end_time } = self;
        format!("api=SYNO.FotoTeam.Browse.Item&method=list&version=1&offset={offset}&limit={limit}&start_time={start_time}&end_time={end_time}")
    }
}


#[derive(Debug, Clone)]
pub struct AlbumRequest {
    pub offset: u32,
    pub limit: u32,
    pub album_id: AlbumId,
}

impl Request for AlbumRequest {
    type Response = ListResponse;

    fn query(&self) -> String {
        let Self { offset, limit, album_id } = self;
        let album_id = match album_id {
            AlbumId::Owned(id)     => format!("album_id={id}"),
            AlbumId::Shared(pass) => format!("passphrase={pass}"),
        };
        format!("api=SYNO.Foto.Browse.Item&method=list&version=1&{album_id}&offset={offset}&limit={limit}")
    }
}