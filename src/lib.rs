pub const BLOCK_SIZE: usize = 16;

pub async fn check_token(token: &[u8]) -> String {
    let b64   = base64::encode_config(token, base64::URL_SAFE);
    let res   = reqwest::get(format!("http://localhost:4567/check?token={b64}")).await.unwrap();
    let bytes = res.bytes().await.unwrap();

    String::from_utf8(bytes.to_vec()).unwrap()
}

pub async fn fetch_token() -> Vec<u8> {
    let res   = reqwest::get("http://localhost:4567/token").await.unwrap();
    let token = res.bytes().await.unwrap();

    base64::decode_config(token, base64::URL_SAFE).unwrap()
}

