use padding_oracle_attack::*;

#[tokio::main]
async fn main() {
    let token = fetch_token().await;
    println!("\n=== bytes ===");
    println!("{:02x?}", &token);

    let admin_token: Vec<u8> = disguised_admin(token.clone());

    let mut msg = check_token(&admin_token).await;
    println!("\n=== response ===");
    println!("{}", &msg);


    let tampered_token: Vec<u8> = append_value(token.clone(), b" \"flag\": true }");

    msg = check_token(&tampered_token).await;
    println!("\n=== response ===");
    println!("{}", &msg);
}

fn disguised_admin(mut token: Vec<u8>) -> Vec<u8> {
    token[6] ^= b'9';
    token[6] ^= b'0';

    token
}

fn append_value(mut token:Vec<u8>, value: &[u8]) -> Vec<u8> {
    let mut value_vec: Vec<u8> = value.to_vec();

    token[21] ^= b'}';
    token[21] ^= b',';

    token.append(&mut value_vec);

    token
}
