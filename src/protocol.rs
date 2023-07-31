use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A collection whose events can be subscribed to.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Collection {
    /// Collection with slug.
    Collection(String),
    /// All possible collections.
    All,
}

impl Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "collection:{}",
            match &self {
                Collection::Collection(c) => c,
                Collection::All => "*",
            }
        )
    }
}

/// The websocket to connect to.
///
/// OpenSea provides two websockets for either `Mainnet` (production) networks for `Testnet` networks.
/// See [`Chain`](crate::schema::Chain) for a full list of supported chains.
/// #[derive(Debug)]
pub enum Network {
    /// Mainnet (`Ethereum`, `Polygon`, `Klaytn`, `Solana`)
    Mainnet,
    /// Testnet (`Goerli`, `Mumbai`, `Baobab`)
    Testnet,
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Network::Mainnet => write!(f, "wss://stream.openseabeta.com/socket/websocket"),
            Network::Testnet => write!(f, "wss://testnets-stream.openseabeta.com/socket/websocket"),
        }
    }
}

/// Receivable events from the websocket.
///
/// This type belongs to the `event` field of [`Message`](phyllo::message::Message), not to be confused with
/// [`Payload`](crate::schema::Payload).
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Event {
    /// An item been listed for sale.
    ItemListed,
    /// An item has been sold.
    ItemSold,
    /// An item has been transferred from one wallet to another.
    ItemTransferred,
    /// An item has had its metadata updated.
    ItemMetadataUpdated,
    /// An item has had its listing cancelled.
    ItemCancelled,
    /// An item has received an offer.
    ItemReceivedOffer,
    /// An item has received a bid.
    ItemReceivedBid,
    /// A collection has received an offer.
    CollectionOffer,
}
