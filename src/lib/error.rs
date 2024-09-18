#[derive(std::fmt::Debug)]
pub enum Error {
    NostrError(nostr_sdk::client::Error)
}

impl From<nostr_sdk::client::Error> for Error {
    fn from(e: nostr_sdk::client::Error) -> Error {
        Error::NostrError(e)
    }
}