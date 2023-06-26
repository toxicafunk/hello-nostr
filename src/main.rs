use nostr_sdk::prelude::*;
use std::str::FromStr;

const PRIVATE_KEY: &str = "dab63d8ad6442c28192788a4febe00cea85c1c6a2da84e53745b1c15e0861735";

#[tokio::main]
async fn main() -> Result<()> {

    let secret_key = SecretKey::from_str(PRIVATE_KEY).unwrap();
    let my_keys = Keys::new(secret_key);

    println!("Hello , nostr! Ny public key is: {}", my_keys.public_key().to_string());

    let client = Client::new(&my_keys);
    client.add_relay("wss://relay.house", None).await?;
    client.add_relay("wss://relay.damus.io", None).await?;
    client.connect().await;

    Ok(())
}
