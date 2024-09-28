use crate::error::Error;
use nostr_sdk::prelude::*;
use tracing::{error, info};

pub async fn publish(relay_url: &str, keys: Keys, mined_event: UnsignedEvent) -> Result<(), Error> {
    let client = ClientBuilder::new().signer(keys).build();
    client.add_relay(relay_url).await?;

    let signer = client.signer().await?;
    let signed_mined_event = signer
        .sign_event(mined_event)
        .await
        .expect("expect successful signature");

    // Connect to relays
    info!("connecting to relay: {}", relay_url);
    client.connect().await;

    match client.send_event(signed_mined_event).await {
        Ok(send_output) => info!("send mined event output: {:?}", send_output),
        Err(e) => error!("failed to send mined event: {}", e),
    };

    Ok(())
}
