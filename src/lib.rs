use reqwest::{Client, Url};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
type DateTime = chrono::DateTime<chrono::Utc>;
use chrono::serde::ts_seconds;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    pub success: bool,
    #[serde(flatten)]
    pub body: Body<T>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Body<T> {
    Error(ErrorResponse),
    Data (T),
}

impl<T> Body<T> {
    pub fn as_result(self) -> Result<T, ErrorResponse> {
        match self {
            Body::Error(error) => Err(error),
            Body::Data(data) => Ok(data),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    code: u32,
}

pub trait Request {
    type Response: DeserializeOwned;
    fn query(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct LoginRequest {
    pub account: String,
    pub passwd: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub did: String,
    pub sid: String
}

impl Request for LoginRequest {
    type Response = LoginResponse;
    fn query(&self) -> String {
        format!("api=SYNO.API.Auth&version=3&method=login&account={}&passwd={}", self.account, self.passwd)
    }
}

#[derive(Debug)]
pub enum SynoError {
    Syno(ErrorResponse),
    Reqwest(reqwest::Error),
}

impl From<reqwest::Error> for SynoError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}
impl From<ErrorResponse> for SynoError {
    fn from(value: ErrorResponse) -> Self {
        Self::Syno(value)
    }
}

type SynoResult<T> = Result<T, SynoError>;

pub struct SynoService {
    client: Client,
    api_url: Url,
    sid: String,
}

impl SynoService {
    /**
     * api_url - for example, http://192.168.1.199:5000/webapi/entry.cgi
     */
    pub async fn login(client: Client, root_url: impl reqwest::IntoUrl, login_request: LoginRequest) -> SynoResult<SynoService> {
        let api_url = root_url.into_url()?;
        let response = client.post(api_url.clone())
            .body(login_request.query())
            .send().await?
            .json::<Response<LoginResponse>>().await?;
        let sid = response.body.as_result()?.sid;
        Ok(Self { client, api_url, sid})
    }

    pub async fn request<R: Request>(&self, request: R) -> SynoResult<R::Response> {
        let query = format!("{}&_sid={}", request.query(), self.sid);
        log::trace!("query: {query}");
        let response = self.client
            .post(self.api_url.as_str())
            .body(query).send().await?
            .json::<Response<R::Response>>().await?;
        match response.body {
            Body::Error(e) => Err(SynoError::Syno(e)),
            Body::Data(d) => Ok(d),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BrowseItemListRequest {
    pub offset: u64,
    pub limit: u64,
    pub start_time: DateTime,
    pub end_time: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemList{
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

impl Request for BrowseItemListRequest {
    type Response = ItemList;

    fn query(&self) -> String {
        format!("api=SYNO.FotoTeam.Browse.Item&method=list&version=1&offset={}&limit={}&start_time={}&end_time={}",
            self.offset, self.limit, self.start_time.timestamp(), self.end_time.timestamp()
        )
    }
}
