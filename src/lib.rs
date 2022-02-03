pub const BLOCK_SIZE: usize = 16;

pub async fn fetch_token() -> Vec<u8> {
    let res   = reqwest::get("http://localhost:4567/token").await.unwrap();
    let token = res.bytes().await.unwrap();

    base64::decode_config(token, base64::URL_SAFE).unwrap()
}

