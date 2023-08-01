use chrono::prelude::*;
use opensea_stream::client::Client;
use opensea_stream::protocol::{Collection, Network};
use opensea_stream::schema;
use std::env;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let api_key = &args[1];

    let mut client = Client::new(Network::Mainnet, api_key).await;
    client.subscribe(Collection::All).await;

    let timer = Instant::now();
    let mut counter_item_listed: u64 = 0;
    let mut counter_item_sold: u64 = 0;
    let mut counter_item_transfered: u64 = 0;
    let mut counter_item_metadata_updated: u64 = 0;
    let mut counter_item_cancelled: u64 = 0;
    let mut counter_item_received_offer: u64 = 0;
    let mut counter_item_received_bid: u64 = 0;
    let mut counter_item_collection_offer: u64 = 0;
    let mut counter_item_trait_offer: u64 = 0;
    let mut counter_order_invalidate: u64 = 0;
    let mut counter_order_revalidate: u64 = 0;

    println!("{:>8} | {:>8} | {:>8} | {:>8} | {:>8} | {:>8} | {:>8} | {:>8} | {:>8} | {:>8} | {:>8} | {:>8}",
        "listings", "sold", "transfer", "metadata", "cancel", "offer", "bid", "c_offer", "t_offer", "invalid", "revalid", "total");

    let mut second = Utc::now().second();
    loop {
        let event = client.read_event().await;

        let event = match event {
            Some(v) => v,
            _ => {
                continue;
            }
        };

        match event.payload {
            schema::Payload::ItemListed(_item) => {
                counter_item_listed += 1;
            }
            schema::Payload::ItemSold(_item) => {
                counter_item_sold += 1;
            }
            schema::Payload::ItemTransferred(_item) => {
                counter_item_transfered += 1;
            }
            schema::Payload::ItemMetadataUpdated(_item) => {
                counter_item_metadata_updated += 1;
            }
            schema::Payload::ItemCancelled(_item) => {
                counter_item_cancelled += 1;
            }
            schema::Payload::ItemReceivedOffer(_item) => {
                counter_item_received_offer += 1;
            }
            schema::Payload::ItemReceivedBid(_item) => {
                counter_item_received_bid += 1;
            }
            schema::Payload::CollectionOffer(_item) => {
                counter_item_collection_offer += 1;
            }
            schema::Payload::TraitOffer(_item) => {
                counter_item_trait_offer += 1;
            }
            schema::Payload::OrderInvalidate(_item) => {
                counter_order_invalidate += 1;
            }
            schema::Payload::OrderRevalidate(_item) => {
                counter_order_revalidate += 1;
            }
            _ => {
                println!("other event: {:?}", event.payload)
            }
        }

        let act_second = Utc::now().second();

        if second != act_second {
            let total = counter_item_listed
                + counter_item_sold
                + counter_item_transfered
                + counter_item_metadata_updated
                + counter_item_cancelled
                + counter_item_received_offer
                + counter_item_received_bid
                + counter_item_collection_offer
                + counter_item_trait_offer
                + counter_order_invalidate
                + counter_order_revalidate;
            println!(
                "{:>6}/s | {:>6}/s | {:>6}/s | {:>6}/s | {:>6}/s | {:>6}/s | {:>6}/s | {:>6}/s | {:>6}/s | {:>6}/s | {:>6}/s | {:>6}/s",
                format!("{:.2}", (counter_item_listed as f64 / timer.elapsed().as_millis() as f64) * 1000.0),
                format!("{:.2}", (counter_item_sold as f64 / timer.elapsed().as_millis() as f64) * 1000.0),
                format!("{:.2}", (counter_item_transfered as f64 / timer.elapsed().as_millis() as f64) * 1000.0),
                format!("{:.2}", (counter_item_metadata_updated as f64 / timer.elapsed().as_millis() as f64) * 1000.0),
                format!("{:.2}", (counter_item_cancelled as f64 / timer.elapsed().as_millis() as f64) * 1000.0),
                format!("{:.2}", (counter_item_received_offer as f64 / timer.elapsed().as_millis() as f64) * 1000.0),
                format!("{:.2}", (counter_item_received_bid as f64 / timer.elapsed().as_millis() as f64) * 1000.0),
                format!("{:.2}", (counter_item_collection_offer as f64 / timer.elapsed().as_millis() as f64) * 1000.0),
                format!("{:.2}", (counter_item_trait_offer as f64 / timer.elapsed().as_millis() as f64) * 1000.0),
                format!("{:.2}", (counter_order_invalidate as f64 / timer.elapsed().as_millis() as f64) * 1000.0),
                format!("{:.2}", (counter_order_revalidate as f64 / timer.elapsed().as_millis() as f64) * 1000.0),
                format!("{:.2}", (total as f64 / timer.elapsed().as_millis() as f64) * 1000.0),
            );
            second = act_second;
        }
    }
}
