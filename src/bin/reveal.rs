use padding_oracle_attack::*;

#[tokio::main]
async fn main() {
    let token = fetch_token().await;
    println!("\n=== token ===");
    println!("{:02x?}", &token);

    println!("\n=== original data ===");
    println!("{:02x?}", debug::decrypt_token(&token));

    println!("\n=== plain text ===");
    let plain_text: Vec<u8> = reveal_plain_text(&token).await;
    println!("\n{:02x?}", &plain_text);
    println!("\n{}", String::from_utf8_lossy(&plain_text));
}

async fn reveal_plain_text(token: &Vec<u8>) -> Vec<u8> {
    let block_count = token.len() / BLOCK_SIZE;
    let mut plain_text: Vec<u8> = Vec::new();

    // block size check
    if token.len() % BLOCK_SIZE != 0 {
        panic!("illegal block size")
    }

    for block_offset in 1..block_count {
        let mut modified_token: Vec<u8> = token[..(token.len() - BLOCK_SIZE*(block_offset - 1))].to_vec();
        for byte_offset in 1..=BLOCK_SIZE {
            for i in (0u8..255).rev() { // try the original value last ( token ^ i: 255 ~ 0 )
                let byte_index = BLOCK_SIZE*block_count - BLOCK_SIZE*block_offset - byte_offset;
    
                modified_token[byte_index] = token[byte_index] ^ i;
    
                let msg = check_token(&modified_token).await;
                if msg != "decrypt error" {
                    let m: u8 = token[byte_index] ^ modified_token[byte_index] ^ (byte_offset as u8);
                    plain_text.insert(0, m);
                    println!("\n{:02x?}", modified_token);
                    println!("[debug] m = {:02x?}", m);
    
                    for j in 0..byte_offset {
                        modified_token[byte_index + j] ^= byte_offset as u8;     // reset value
                        modified_token[byte_index + j] ^= byte_offset as u8 + 1; // set next padding
                    }
    
                    break;
                }
            }
        }
    }

    plain_text
}
