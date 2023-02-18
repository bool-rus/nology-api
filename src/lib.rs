use reqwest::{Client, Url};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

pub mod album;
pub mod browse;
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
