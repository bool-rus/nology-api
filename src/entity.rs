
#[derive(Debug, Clone)]
pub enum AlbumId {
    Owned(i64),
    Shared(String),
}
