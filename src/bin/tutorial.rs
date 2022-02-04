use padding_oracle_attack::*;

#[tokio::main]
async fn main() {
    let token = fetch_token().await;
    println!("\n=== bytes ===");
    println!("{:02x?}", &token);

    let admin_token: Vec<u8> = disguised_admin(token.clone());

    let msg = check_token(&admin_token).await;
    println!("\n=== response ===");
    println!("{}", &msg);
}

fn disguised_admin(mut token: Vec<u8>) -> Vec<u8> {
    token[6] ^= b'9';
    token[6] ^= b'0';

    token
}
