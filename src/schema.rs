use crate::protocol::Event;
use chrono::{DateTime, Utc};
use ethers_core::{
    abi::Address,
    types::{H256, U256},
};
use serde::{de::Error, Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};
use std::{fmt, str::FromStr};
use url::Url;

/// Payload of a message received from the websocket.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamEvent {
    /// Timestamp of when this message was sent to the client.
    pub sent_at: DateTime<Utc>,
    /// Contents of the message
    #[serde(flatten)]
    pub payload: Payload,
}

/// Content of the message.
///
/// This type corresponds to the JSON objects recieved [as described here](https://docs.opensea.io/reference/stream-api-event-schemas),
/// not the event type used for the Phoenix protocol (see [`Event`]).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event_type", content = "payload")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Payload {
    /// An item has been listed for sale.
    ItemListed(ItemListedData),
    /// An item has been sold.
    ItemSold(ItemSoldData),
    /// An item has been transferred from one wallet to another.
    ItemTransferred(ItemTransferredData),
    /// An item has had its metadata updated.
    ItemMetadataUpdated(ItemMetadataUpdatedData),
    /// An item has had its listing cancelled.
    ItemCancelled(ItemCancelledData),
    /// An item has received an offer.
    ItemReceivedOffer(ItemReceivedOfferData),
    /// An item has received a bid.
    ItemReceivedBid(ItemReceivedBidData),
    /// A collection has received an offer.
    CollectionOffer(CollectionOfferData),
    /// A trait has received an offer.
    TraitOffer(TraitOfferData),
    /// An order has been invalidated.
    OrderInvalidate(OrderInvalidateData),
    /// An order has been revalidated.
    OrderRevalidate(OrderRevalidateData),
}

impl From<Payload> for Event {
    fn from(val: Payload) -> Self {
        match val {
            Payload::ItemListed(_) => Event::ItemListed,
            Payload::ItemSold(_) => Event::ItemSold,
            Payload::ItemTransferred(_) => Event::ItemTransferred,
            Payload::ItemMetadataUpdated(_) => Event::ItemMetadataUpdated,
            Payload::ItemCancelled(_) => Event::ItemCancelled,
            Payload::ItemReceivedOffer(_) => Event::ItemReceivedOffer,
            Payload::ItemReceivedBid(_) => Event::ItemReceivedBid,
            Payload::CollectionOffer(_) => Event::CollectionOffer,
            Payload::TraitOffer(_) => Event::TraitOffer,
            Payload::OrderInvalidate(_) => Event::OrderInvalidate,
            Payload::OrderRevalidate(_) => Event::OrderRevalidate,
        }
    }
}

/// A collection on OpenSea.
#[derive(Debug, Clone)]
pub struct Collection(String);

impl Serialize for Collection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Inner {
            slug: String,
        }

        Inner {
            slug: self.0.clone(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Collection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner {
            slug: String,
        }

        Deserialize::deserialize(deserializer).map(|v: Inner| Collection(v.slug))
    }
}

/// Context about an item.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    /// Identifier.
    pub nft_id: Option<NftId>,
    /// Link to OpenSea page.
    pub permalink: Option<Url>,
    /// Chain the item is on.
    pub chain: Option<Chain>,
    /// Basic metadata.
    pub metadata: Option<Metadata>,
}

/// Identifier of the NFT.
#[derive(Debug, Clone)]
pub struct NftId {
    /// Chain the item is on.
    pub network: Chain,
    /// Contract address.
    pub address: Address,
    /// Token ID.
    pub id: String,
}

impl Serialize for NftId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{}/{:?}/{}", self.network, self.address, self.id).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NftId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let mut parts = s.splitn(3, '/').fuse();

        let network = parts
            .next()
            .map(Chain::from_str)
            .ok_or_else(|| D::Error::custom("expected network"))?
            .map_err(|_| D::Error::custom("invalid network"))?;

        let address = parts
            .next()
            .map(Address::from_str)
            .ok_or_else(|| D::Error::custom("expected address"))?
            .map_err(D::Error::custom)?;

        let id = parts.next().map(String::from).unwrap();

        Ok(NftId {
            network,
            address,
            id,
        })
    }
}

mod chain {
    #![allow(deprecated)]
    use serde::{Deserialize, Serialize};

    /// Network an item is on.
    #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
    #[serde(tag = "name", rename_all = "lowercase")]
    #[non_exhaustive]
    pub enum Chain {
        /// [Avalanche](https://www.avalabs.org/) mainnet.
        Avalanche,
        /// [Base] (https://base.org/) mainnet
        Base,
        /// [BSC](https://www.bnbchain.org/en) mainnet.
        Bsc,
        /// [Ethereum](https://ethereum.org) mainnet.
        Ethereum,
        /// Optimism
        Optimism,
        /// Arbitrum
        Arbitrum,
        // Arbitrum Nova
        #[serde(rename = "arbitrum_nova")]
        ArbitrumNova,
        /// [Polygon](https://polygon.technology/solutions/polygon-pos) mainnet.
        #[serde(rename = "matic")]
        Polygon,
        /// [Klaytn](https://www.klaytn.foundation/) mainnet.
        Klaytn,
        /// [Solana](https://solana.com/) mainnet. This variant (and all events for Solana assets) are not supported in this version.
        Solana,
        /// [Goerli](https://ethereum.org/en/developers/docs/networks/#goerli) testnet (of Ethereum).
        Goerli,
        /// [Mumbai](https://docs.polygon.technology/docs/develop/network-details/network#mumbai-pos-testnet) testnet (of Polygon).
        Mumbai,
        /// [Baobab](https://www.klaytn.foundation/) testnet (of Klaytn).
        Baobab,
        /// [Zora](https://zora.co/) mainnet.
        Zora,
    }
}
pub use chain::Chain;

impl FromStr for Chain {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "avalanche" => Ok(Chain::Avalanche),
            "base" => Ok(Chain::Base),
            "bsc" => Ok(Chain::Bsc),
            "ethereum" => Ok(Chain::Ethereum),
            "optimism" => Ok(Chain::Optimism),
            "arbitrum" => Ok(Chain::Arbitrum),
            "arbitrum_nova" => Ok(Chain::ArbitrumNova),
            "matic" => Ok(Chain::Polygon),
            "klaytn" => Ok(Chain::Klaytn),
            "solana" => Ok(Chain::Solana),
            "mumbai" => Ok(Chain::Mumbai),
            "baobab" => Ok(Chain::Baobab),
            "zora" => Ok(Chain::Zora),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Chain::Avalanche => "avalanche",
                Chain::Base => "base",
                Chain::Bsc => "bsc",
                Chain::Ethereum => "ethereum",
                Chain::Optimism => "optimism",
                Chain::Arbitrum => "arbitrum",
                Chain::ArbitrumNova => "arbitrum_nova",
                Chain::Polygon => "matic",
                Chain::Klaytn => "klaytn",
                Chain::Solana => "solana",
                Chain::Mumbai => "mumbai",
                Chain::Baobab => "baobab",
                Chain::Goerli => "goerli",
                Chain::Zora => "zora",
            }
        )
    }
}

/// Basic metadata of an item.
///
/// This is fetched directly from an item's metadata according to [metadata standards](https://docs.opensea.io/docs/metadata-standards).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    /// Animation Url
    pub animation_url: Option<Url>,
    /// Background color
    pub background_color: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Image preview URL.
    pub image_preview_url: Option<Url>,
    /// Image URL. This is shown on the collection's storefront.
    pub image_url: Option<Url>,
    /// URL to metadata.
    pub metadata_url: Option<Url>,
    /// external link
    pub external_link: Option<Url>,
    /// Name.
    pub name: Option<String>,
    // traits
    pub traits: Option<Vec<Trait>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trait {
    pub trait_type: String,
    pub value: Option<String>,
    pub display_type: Option<String>,
    pub max_value: Option<u64>,
    pub trait_count: Option<u64>,
    pub order: Option<u64>,
}

/// Payload data for [`Payload::ItemListed`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemListedData {
    /// Collection that the token belongs to.
    pub collection: Collection,
    /// Information about the item itself.
    pub item: Item,
    /// Timestamp of when the listing was created.
    pub event_timestamp: DateTime<Utc>,
    /// Starting price of the listing. See `payment_token` for the actual value of each unit.
    #[serde(with = "u256_fromstr_radix_10")]
    pub base_price: U256,
    /// Expiration date.
    pub expiration_date: DateTime<Utc>,
    /// Whether the listing is private.
    pub is_private: bool,
    /// Timestamp of when the listing was created.
    pub listing_date: DateTime<Utc>,
    /// Type of listing. `None` indicates the listing is a buyout.
    pub listing_type: Option<ListingType>,
    /// Creator of the listing.
    #[serde(with = "address_fromjson")]
    pub maker: Address,
    /// Hash id of the listing.
    pub order_hash: H256,
    /// Token accepted for payment.
    pub payment_token: PaymentToken,
    /// protocol data from OS
    pub protocol_data: ProtocolData,
    // /// Number of items on sale. This is always `1` for ERC-721 tokens.
    // pub quantity: u64,
    // /// Buyer of the listing.
    // #[serde(with = "address_fromjson_opt", default)]
    // pub taker: Option<Address>,
}

/// Payload data for [`Payload::ItemSold`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemSoldData {
    /// Collection that the token belongs to.
    pub collection: Collection,
    /// Information about the item itself.
    pub item: Item,

    /// Timestamp of when the listing was closed.
    pub closing_date: DateTime<Utc>,
    /// Timestamp of when the item was sold.
    pub event_timestamp: DateTime<Utc>,
    /// Whether the listing was private.
    pub is_private: bool,
    /// Type of listing. `None` indicates the listing was a buyout.
    pub listing_type: Option<ListingType>,
    /// Creator of the listing.
    #[serde(with = "address_fromjson")]
    pub maker: Address,
    /// Hash id of the listing.
    pub order_hash: H256,
    /// Token used for payment.
    pub payment_token: PaymentToken,
    /// Number of items bought. This is always `1` for ERC-721 tokens.
    pub quantity: u64,
    /// Purchase price. See `payment_token` for the actual value of each unit.
    #[serde(with = "u256_fromstr_radix_10")]
    pub sale_price: U256,
    /// Buyer/winner of the listing.
    #[serde(with = "address_fromjson")]
    pub taker: Address,
    /// Transaction for the purchase.
    pub transaction: Transaction,
}

/// Payload data for [`Payload::ItemTransferred`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemTransferredData {
    //// Collection that the token belongs to.
    pub collection: Collection,
    /// Timestamp of when the item was transferred.
    pub event_timestamp: DateTime<Utc>,
    /// Address the item was transferred from.
    #[serde(with = "address_fromjson")]
    pub from_account: Address,
    /// Information about the item itself.
    pub item: Item,
    // TODO fix this
    // /// Number of items transferred. This is always `1` for ERC-721 tokens.
    // pub quantity: serde_json::Value,
    /// Address the item was transferred to.
    #[serde(with = "address_fromjson")]
    pub to_account: Address,
    /// Transaction of the transfer.
    pub transaction: Option<Transaction>,
}

/// Payload data for [`Payload::ItemMetadataUpdated`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemMetadataUpdatedData {
    //// Collection that the token belongs to.
    pub collection: Collection,
    /// Information about the item itself.
    pub item: Item,
}

/// Payload data for [`Payload::ItemCancelled`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemCancelledData {
    /// Offer price. See `payment_token` for the actual value of each unit.
    #[serde(with = "u256_fromstr_radix_10")]
    pub base_price: U256,
    //// Collection that the token belongs to.
    pub collection: Collection,
    /// Timestamp of when the listing was cancelled.
    pub event_timestamp: DateTime<Utc>,
    /// if this a private order/listing
    pub is_private: bool,
    /// Information about the item itself.
    pub item: Item,
    /// Timestamp of when the listing was created.
    pub listing_date: Option<DateTime<Utc>>,
    /// Type of listing. `None` indicates the listing would've been a buyout.
    pub listing_type: Option<ListingType>,
    // TODO find out if maker is allways null
    // /// Creator of the cancellation order.
    // #[serde(with = "address_fromjson")]
    // pub maker: Option<Address>,
    /// Hash id of the listing.
    pub order_hash: H256,
    /// Token accepted for payment.
    pub payment_token: PaymentToken,
    /// Number of items in listing. This is always `1` for ERC-721 tokens.
    pub quantity: u64,
    /// Transaction for the cancellation.
    pub transaction: Option<Transaction>,
}

/// Payload data for [`Payload::ItemReceivedOffer`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemReceivedOfferData {
    /// Collection that the token belongs to.
    pub collection: Collection,
    /// Information about the item itself.
    pub item: Item,

    /// Timestamp of when the offer was received.
    pub event_timestamp: DateTime<Utc>,
    /// Offer price. See `payment_token` for the actual value of each unit.
    #[serde(with = "u256_fromstr_radix_10")]
    pub base_price: U256,
    /// Timestamp of when the offer was created.
    pub created_date: DateTime<Utc>,
    /// Timestamp of when the offer will expire.
    pub expiration_date: DateTime<Utc>,
    /// Creator of the offer.
    #[serde(with = "address_fromjson")]
    pub maker: Address,
    /// Hash id of the listing.
    pub order_hash: H256,
    /// Token offered for payment.
    pub payment_token: PaymentToken,
    /// Number of items on the offer. This is always `1` for ERC-721 tokens.
    pub quantity: u64,
    /// Taker of the offer.
    #[serde(with = "address_fromjson_opt", default)]
    pub taker: Option<Address>,
}

/// Payload data for [`Payload::ItemReceivedBid`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemReceivedBidData {
    /// Collection that the token belongs to.
    pub collection: Collection,
    /// Information about the item itself.
    pub item: Item,

    /// Timestamp of when the bid was received.
    pub event_timestamp: DateTime<Utc>,
    /// Bid price. See `payment_token` for the actual value of each unit.
    #[serde(with = "u256_fromstr_radix_10")]
    pub base_price: U256,
    /// Timestamp of when the bid was created.
    pub created_date: DateTime<Utc>,
    /// Timestamp of when the bid will expire.
    pub expiration_date: DateTime<Utc>,
    /// Creator of the bid.
    #[serde(with = "address_fromjson")]
    pub maker: Address,
    /// Hash id of the listing.
    pub order_hash: H256,
    /// Token offered for payment.
    pub payment_token: PaymentToken,
    /// Number of items on the offer. This is always `1` for ERC-721 tokens.
    pub quantity: u64,
    /// Taker of the bid.
    #[serde(with = "address_fromjson_opt", default)]
    pub taker: Option<Address>,
}

/// Payload data for [`Payload::CollectionOffer`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionOfferData {
    /// Asset contract criteria.
    #[serde(with = "address_fromjson")]
    pub asset_contract_criteria: Address,
    /// Bid price. See `payment_token` for the actual value of each unit.
    #[serde(with = "u256_fromstr_radix_10")]
    pub base_price: U256,
    //// Collection that the token belongs to.
    pub collection: Collection,
    /// Collection criteria.
    pub collection_criteria: CollectionCriteria,
    /// Timestamp of when the bid was created.
    pub created_date: DateTime<Utc>,
    /// Timestamp of when the bid was received.
    pub event_timestamp: DateTime<Utc>,
    /// Timestamp of when the bid will expire.
    pub expiration_date: DateTime<Utc>,
    /// Creator of the bid.
    #[serde(with = "address_fromjson")]
    pub maker: Address,
    /// Hash id of the listing.
    pub order_hash: H256,
    /// Token offered for payment.
    pub payment_token: PaymentToken,
    /// the address of the used zone
    pub protocol_address: Address,
    /// the protocol data from OS
    pub protocol_data: ProtocolData,
    /// Number of items on the offer. This is always `1` for ERC-721 tokens.
    pub quantity: u64,
    /// Taker of the bid.
    #[serde(with = "address_fromjson_opt", default)]
    pub taker: Option<Address>,
}

/// Payload data for [`Payload::TraitOffer`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraitOfferData {
    /// Asset contract criteria.
    #[serde(with = "address_fromjson")]
    pub asset_contract_criteria: Address,
    /// Bid price. See `payment_token` for the actual value of each unit.
    #[serde(with = "u256_fromstr_radix_10")]
    pub base_price: U256,
    //// Collection that the token belongs to.
    pub collection: Collection,
    /// Collection criteria.
    pub collection_criteria: CollectionCriteria,
    /// Timestamp of when the bid was created.
    pub created_date: DateTime<Utc>,
    /// Timestamp of when the bid was received.
    pub event_timestamp: DateTime<Utc>,
    /// Timestamp of when the bid will expire.
    pub expiration_date: DateTime<Utc>,
    /// Creator of the bid.
    #[serde(with = "address_fromjson")]
    pub maker: Address,
    /// Hash id of the listing.
    pub order_hash: H256,
    /// Token offered for payment.
    pub payment_token: PaymentToken,
    /// the address of the used zone
    pub protocol_address: Address,
    /// the protocol data from OS
    pub protocol_data: ProtocolData,
    /// Number of items on the offer. This is always `1` for ERC-721 tokens.
    pub quantity: u64,
    /// Taker of the bid.
    #[serde(with = "address_fromjson_opt", default)]
    pub taker: Option<Address>,
    /// the traits of the given offer
    pub trait_criteria: TraitCriteria,
}

/// Payload data for [`Payload::OrderInvalidate`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderInvalidateData {
    /// Asset contract criteria.
    pub chain: Chain,
    //// Collection that the token belongs to.
    pub collection: Collection,
    /// Timestamp of when the bid was received.
    pub event_timestamp: DateTime<Utc>,
    /// Information about the item itself.
    pub item: Item,
    /// Hash id of the listing.
    pub order_hash: Option<H256>,
    /// the address of the used zone
    pub protocol_address: Address,
}

// pub enum Address {
//     /// an ethereum address
//     Ethereum(abi::Address),
//     /// a solana address
//     Solana(String),
// }
/// Payload data for [`Payload::OrderRevalidate`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderRevalidateData {
    /// Asset contract criteria.
    pub chain: Chain,
    //// Collection that the token belongs to.
    pub collection: Collection,
    /// Timestamp of when the bid was received.
    pub event_timestamp: DateTime<Utc>,
    /// Information about the item itself.
    pub item: Item,
    /// Hash id of the listing.
    pub order_hash: H256,
    /// the address of the used zone
    pub protocol_address: Address,
}

/// the criteria for the collection
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionCriteria {
    pub slug: String,
}

/// the criteria for the trait
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraitCriteria {
    pub trait_name: String,
    pub trait_type: String,
}

/// Auctioning system used by the listing.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ListingType {
    /// [English](https://en.wikipedia.org/wiki/English_auction) (ascending).
    English,
    /// [Dutch](https://en.wikipedia.org/wiki/Dutch_auction) (descending).
    Dutch,
}

impl fmt::Display for ListingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ListingType::English => "English",
                ListingType::Dutch => "Dutch",
            }
        )
    }
}

/// Details of a transaction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    /// Transaction hash
    pub hash: H256,
    /// Timestamp of transaction
    pub timestamp: DateTime<Utc>,
}

/// Token used for payment.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentToken {
    /// Contract address
    pub address: Address,
    /// Granularity of the token
    pub decimals: u64,
    /// Price of token (denominated in ETH)
    #[serde(with = "f64_fromstring")]
    pub eth_price: f64,
    /// Name
    pub name: String,
    /// Symbol
    pub symbol: String,
    /// Price of token (denominated in USD)
    #[serde(with = "f64_fromstring")]
    pub usd_price: f64,
}

/// Protocol data for offers and item transfers.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolData {
    /// the protocol parameters of the event
    pub parameters: Parameters,
    /// the signature from the counterparty
    pub signature: Option<String>,
}

/// the parameters of the event
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    /// the conduit key for this listing
    pub conduit_key: String,
    /// the consideration items for the payments
    pub consideration: Vec<Consideration>,
    /// a counter
    pub counter: serde_json::Value,
    /// the end time for the listing
    #[serde(with = "timestamp_to_date")]
    pub end_time: DateTime<Utc>,
    /// the offer object itself
    pub offer: Vec<Offer>,
    /// the offerer
    pub offerer: Address,
    /// the OS order type
    pub order_type: u64,
    /// random salt
    pub salt: String,
    /// the start time of the listing
    #[serde(with = "timestamp_to_date")]
    pub start_time: DateTime<Utc>,
    /// the amount of consideration items
    pub total_original_consideration_items: u64,
    /// the zone for the execution (post execution evaluation)
    pub zone: Address,
    /// the hash of the given zone
    pub zone_hash: String,
}

/// a consideration item for an offer
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Consideration {
    /// the type of the given transfer
    pub item_type: u64,
    /// the address of the offered item
    pub token: Address,
    /// the identifier or criteria of the offer
    pub identifier_or_criteria: String,
    /// the min amount to transfer to the recipient
    pub start_amount: String,
    /// the max amount to transfer to the recipient
    pub end_amount: Option<String>,
    /// the recipient of this transfer
    pub recipient: Address,
}

/// the offer object within the protocol data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    /// the max amount of the offer
    pub end_amount: String,
    /// the identifier or criteria of the offer
    pub identifier_or_criteria: String,
    /// the type of the offered item
    pub item_type: u64,
    /// the min amount of the offer
    pub start_amount: String,
    /// the address of the offered item
    pub token: Address,
}

mod address_fromjson {
    use ethers_core::abi::Address;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    struct Inner {
        address: Address,
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|v: Inner| v.address)
    }

    pub fn serialize<S>(value: &Address, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Inner { address: *value }.serialize(serializer)
    }
}

mod address_fromjson_opt {
    use ethers_core::abi::Address;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    struct Inner {
        address: Address,
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Address>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner: Option<Inner> = Deserialize::deserialize(deserializer)?;
        Ok(inner.map(|i| i.address))
    }

    pub fn serialize<S>(value: &Option<Address>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.map(|v| Inner { address: v }).serialize(serializer)
    }
}

// h/t: meetmangukiya (https://gist.github.com/meetmangukiya/40cad17bcb7d3196d33b072a3500fac7)
mod u256_fromstr_radix_10 {
    use super::*;
    use serde::{de::Visitor, Deserializer, Serializer};
    use std::fmt;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<U256, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Helper;

        impl<'de> Visitor<'de> for Helper {
            type Value = U256;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                U256::from_dec_str(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_str(Helper)
    }

    pub fn serialize<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&value)
    }
}

mod f64_fromstring {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringFloat {
            Str(String),
            F64(f64),
        }

        match StringFloat::deserialize(deserializer)? {
            StringFloat::Str(s) => s.parse().map_err(D::Error::custom),
            StringFloat::F64(f) => Ok(f),
        }
    }

    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.to_string().serialize(serializer)
    }
}

mod timestamp_to_date {
    use chrono::{DateTime, Utc};
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringFloat {
            Str(String),
            Datetime(DateTime<Utc>),
        }

        match StringFloat::deserialize(deserializer)? {
            StringFloat::Datetime(value) => Ok(value),
            StringFloat::Str(value) => {
                let nt = chrono::NaiveDateTime::from_timestamp_opt(value.parse().unwrap(), 0);
                let datetime = DateTime::<Utc>::from_utc(nt.unwrap(), Utc);
                Ok(datetime)
            }
        }
    }

    pub fn serialize<S>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.timestamp().to_string().serialize(serializer)
    }
}
