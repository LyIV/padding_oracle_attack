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

pub mod debug {
    use super::BLOCK_SIZE;

    // ref: https://github.com/kurenaif/padding_oracle_training/blob/1fa4ea4535a011c60080e192ca8769a4473bacf0/problem/app/app.rb#L7
    const KEY: [u8; 32] = [b'a'; 32];

    pub fn decrypt_token(token: &[u8]) -> Vec<u8> {
        let (iv, cipher_text) = token.split_at(BLOCK_SIZE);
        let cipher      = openssl::symm::Cipher::aes_256_cbc();
        let mut decrypt = openssl::symm::Crypter::new(cipher, openssl::symm::Mode::Decrypt, &KEY, Some(iv)).unwrap();
        let mut buffer  = vec![0u8; cipher_text.len() + cipher.block_size()];

        decrypt.pad(false); // disable padding
        decrypt.update(cipher_text, &mut buffer).unwrap();
        decrypt.finalize(&mut buffer).unwrap();

        let truncate_len = buffer.len() - cipher.block_size();
        buffer.truncate(truncate_len);

        buffer
    }
}

