use gloo_net::http::*;

use crate::Result;
use crate::error::Error;

pub async fn get_posts() -> Result<shared::PostBody> {
    let res = Request::get("/api/posts")
        .send().await?;

    into_json(res).await
}

async fn into_json<T>(response: Response) -> Result<T> 
where
    T: serde::de::DeserializeOwned 
{
    if response.ok() {
        response.json().await
            .map_err(|e| e.into())
    } else {
        Err(response.json::<shared::Error>().await
            .map(Error::Api)?)
    }
}
