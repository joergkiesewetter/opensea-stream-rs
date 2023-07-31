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
    let mut counter_order_invalidation: u64 = 0;
    let mut counter_order_revalidation: u64 = 0;

    loop {
        let event = client.read_event().await;

        let event = match event {
            Some(v) => v,
            _ => {
                continue;
            }
        };
        
        match event.payload {
            schema::Payload::ItemListed(listing) => {
                println!("{:#?}", listing);
                counter_item_listed += 1;
            }
            schema::Payload::ItemSold(item) => {
                counter_item_sold += 1;
            }
            schema::Payload::ItemTransferred(item) => {
                counter_item_transfered += 1;
            }
            schema::Payload::ItemMetadataUpdated(item) => {
                counter_item_metadata_updated += 1;
            }
            schema::Payload::ItemCancelled(item) => {
                counter_item_cancelled += 1;
            }
            schema::Payload::ItemReceivedOffer(offer) => {
                counter_item_received_offer += 1;
            }
            schema::Payload::ItemReceivedBid(offer) => {
                counter_item_received_bid += 1;
            }
            schema::Payload::CollectionOffer(offer) => {
                counter_item_collection_offer += 1;
            }
            // schema::Payload::TraitOffer(offer) => {
            //     counter_item_trait_offer += 1;
            // }
            // schema::Payload::OrderInvalidation(order) => {
            //     counter_order_invalidation += 1;
            // }
            // schema::Payload::OrderRevalidation(order) => {
            //     counter_order_revalidation += 1;
            // }
            _ => {
                println!("other event: {:?}", event.payload)
            }
        }

        println!(
            "listings: ({:?}, {:.3}/s); offers: ({:?}, {:.3}/s); cancels: ({:?}, {:.3}/s); sold: ({:?}, {:.3}/s);",
            counter_item_listed,
            (counter_item_listed as f64 / timer.elapsed().as_millis() as f64) * 1000.0,
            counter_item_received_bid,
            (counter_item_received_bid as f64 / timer.elapsed().as_millis() as f64) * 1000.0,
            counter_item_cancelled,
            (counter_item_cancelled as f64 / timer.elapsed().as_millis() as f64) * 1000.0,
            counter_item_sold,
            (counter_item_sold as f64 / timer.elapsed().as_millis() as f64) * 1000.0,
        );
    }

    // loop {
    //     // The message received from the channel is a raw message of the Phoenix protocol.
    //     // It may or may not contain a payload.

    //     // println!("{:?}", subscription.recv().await);
    //     let event: schema::StreamEvent = match subscription.recv().await?.into_custom_payload() {
    //         Some(v) => v,
    //         None => {
    //             eprintln!("unexpected message");
    //             continue;
    //         }
    //     };

    //     // println!("{:?}", event);
    //     // Only print item listing events.
    //     match event.payload {
    //         schema::Payload::ItemListed(listing) => {
    //             println!("{:#?}", listing);
    //             counter_item_listed += 1;
    //         }
    //         schema::Payload::ItemSold(item) => {
    //             counter_item_sold += 1;
    //         }
    //         schema::Payload::ItemTransferred(item) => {
    //             counter_item_transfered += 1;
    //         }
    //         schema::Payload::ItemMetadataUpdated(item) => {
    //             counter_item_metadata_updated += 1;
    //         }
    //         schema::Payload::ItemCancelled(item) => {
    //             counter_item_cancelled += 1;
    //         }
    //         schema::Payload::ItemReceivedOffer(offer) => {
    //             counter_item_received_offer += 1;
    //         }
    //         schema::Payload::ItemReceivedBid(offer) => {
    //             counter_item_received_bid += 1;
    //         }
    //         schema::Payload::CollectionOffer(offer) => {
    //             counter_item_collection_offer += 1;
    //         }
    //         // schema::Payload::TraitOffer(offer) => {
    //         //     counter_item_trait_offer += 1;
    //         // }
    //         // schema::Payload::OrderInvalidation(order) => {
    //         //     counter_order_invalidation += 1;
    //         // }
    //         // schema::Payload::OrderRevalidation(order) => {
    //         //     counter_order_revalidation += 1;
    //         // }
    //         _ => {
    //             println!("other event: {:?}", event.payload)
    //         }
    //     }
    //     // if let schema::Payload::ItemListed(listing) = event.payload {
    //     //     counter_item_listed += 1;
    //     // }

    //     // if let schema::Payload::ItemReceivedOffer(offer) = event.payload {
    //     //     counter_item_offers += 1;
    //     // }

    //     println!(
    //         "listings: ({:?}, {:.3}/s); offers: ({:?}, {:.3}/s); cancels: ({:?}, {:.3}/s); sold: ({:?}, {:.3}/s);",
    //         counter_item_listed,
    //         (counter_item_listed as f64 / timer.elapsed().as_millis() as f64) * 1000.0,
    //         counter_item_received_bid,
    //         (counter_item_received_bid as f64 / timer.elapsed().as_millis() as f64) * 1000.0,
    //         counter_item_cancelled,
    //         (counter_item_cancelled as f64 / timer.elapsed().as_millis() as f64) * 1000.0,
    //         counter_item_sold,
    //         (counter_item_sold as f64 / timer.elapsed().as_millis() as f64) * 1000.0,
    //     );
    // }
}
