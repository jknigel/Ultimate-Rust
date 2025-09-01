//This program is to show the standard error handling vs reqwest error handling

use std::io::{Error, ErrorKind};

async fn my_async_call(url:&str) -> Result<serde_json::Value, Error> {
    let response:reqwest::Response = reqwest::get(url)
    .await
    .map_err(|_| Error::new(ErrorKind::Other, "Could not retrieve response"))?;

    let json_response:serde_json::Value = response
    .json::<serde_json::Value>()
    .await
    .map_err(|_| Error::new(ErrorKind::Other, "Could not decode to JSON"))?;
    
    return Ok(json_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_calls_async_v2_fn() {
        let api_url:&str = "https://pokeapi.co/api/v2/pokemon/ditto";
        let my_res:Result<serde_json::Value, std::io::Error> = my_async_call(api_url).await;
        match my_res {
            Ok(r) => {
                dbg!(r);
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}