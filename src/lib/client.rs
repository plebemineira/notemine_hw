use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use nostr_sdk::prelude::*;
use nostr_sdk::Event;
use tracing::{info, error};
use crate::error::Error;

pub async fn publish(relay_url: &str, mined_event: Event) -> Result<(), Error> {
    let my_keys = Keys::generate();

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9050));
    let connection: Connection = Connection::new()
        .proxy(addr) // Use `.embedded_tor()` instead to enable the embedded tor client (require `tor` feature)
        .target(ConnectionTarget::Onion);

    let opts = Options::new().connection(connection);
    let client = Client::with_opts(&my_keys, opts);

    client.add_relay(relay_url).await?;

    // Connect to relays
    info!("connecting to relay: {}", relay_url);
    client.connect().await;

   match client.send_event(mined_event).await {
       Ok(send_output) => info!("send mined event output: {:?}", send_output),
       Err(e) => error!("failed to send mined event: {}", e),
   };

    Ok(())
}