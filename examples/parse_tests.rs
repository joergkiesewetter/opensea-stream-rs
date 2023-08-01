use serde_json;

use opensea_stream::schema;

pub fn test_parsing() {
    let input = r#"{"event_type":"item_metadata_updated","payload":{"collection":{"slug":"neon-vortex-1"},"item":{"chain":{"name":"matic"},"metadata":{"animation_url":null,"background_color":null,"description":"Neon Vortex NFT coming to unleash dark powers on the Solana","image_url":"https://i.seadn.io/gcs/files/ece163487759d6aa6c768ad9c3aa940b.jpg?w=500&auto=format","metadata_url":"ipfs://bafybeigl23jahosbp7dprqckp72upyl6ivvekextxe4inrz7h3hkiwqydi/3101.json","name":"Neon Vortex #619","traits":[{"display_type":null,"max_value":null,"order":null,"trait_count":0,"trait_type":"Eyes","value":"Navy"},{"display_type":null,"max_value":null,"order":null,"trait_count":0,"trait_type":"Aura","value":"Yin Yang"},{"display_type":"number","max_value":3333,"order":null,"trait_count":0,"trait_type":"Rarity Rank","value":null},{"display_type":null,"max_value":null,"order":null,"trait_count":0,"trait_type":"Outfit","value":"Blue Kimono"},{"display_type":null,"max_value":null,"order":null,"trait_count":0,"trait_type":"Background","value":"Ash"},{"display_type":null,"max_value":null,"order":null,"trait_count":0,"trait_type":"Face","value":"The Neon Vortex"},{"display_type":null,"max_value":null,"order":null,"trait_count":0,"trait_type":"Mask","value":"Samurai"}]},"nft_id":"matic/0x978c92725bb4f87c1da3ba2e8b7c11a24e6aa0a5/3101","permalink":"https://opensea.io/assets/matic/0x978c92725bb4f87c1da3ba2e8b7c11a24e6aa0a5/3101"}},"sent_at":"2023-08-01T22:39:32.033948+00:00"}"#;

    let _json = serde_json::from_str::<schema::StreamEvent>(input).unwrap();
}
