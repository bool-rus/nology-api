# Rust API client for synology/xpenology nas

With some reverse-engineered Api methods

## Features

- Browse items (photo and video)
- Browse albums
- Create albums

If you need to extend features, you [can](#extend)

## Example

```rust,no_run
use nology_api::*;
use chrono::{Utc, Duration};

async fn integration_test() -> SynoResult<()> {
    let account =   env::var("SYNO_USER").unwrap();
    let passwd =    env::var("SYNO_PASS").unwrap();

    //Create service by login
    let service = SynoService::login(
        Default::default(), 
        "http://192.168.1.199:5000/webapi/entry.cgi", 
        LoginRequest { account, passwd }
    ).await?;
    //retrieve last photos (or videos) for 100 days (limit 1000)
    let photos = service.request(browse::ListRequest{
        offset: 0,
        limit: 1000,
        start_time: Utc::now() - Duration::days(100),
        end_time: Utc::now(),
    }).await?; 
    //take 3 last items
    let items = photos.list.into_iter().map(|bi|bi.id).take(3).collect();
    //and create album with it
    let new_album = service.request(album::CreateRequest {
        name: format!("test-album-{}", Utc::now().timestamp()),
        items,
    }).await.unwrap();
    log::info!("album: {new_album:?}");

    Ok(())
}
```

## Extend

You can make your own Request/Response structs, and use it in `SynoService::request` by impementing trait `Request`
