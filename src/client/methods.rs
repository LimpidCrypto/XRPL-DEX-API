extern crate websocket;

use std::collections::HashMap;

use serde_json::Value;
use websocket::{ClientBuilder, Message, OwnedMessage};

use crate::models::normalized_offer;

pub fn transaction_stream() {
    let mut client = ClientBuilder::new("wss://xrplcluster.com/")
        .unwrap()
        .connect(None)
        .unwrap();
    println!("CONNECTED");
    let request = Message::text(
        r#"{
            "id": "Example watch one account and all new ledgers",
            "command": "subscribe",
            "streams": [
              "transactions"
            ]
        }"#,
    );
    client.send_message(&request).unwrap();
    println!("Message sent");
    for message in client.incoming_messages() {
        match message {
            Ok(ref result) => match result {
                OwnedMessage::Text(result) => {
                    let result_json: HashMap<String, Value> = serde_json::from_str(result)
                        .expect("Could not serialize json from `result`.");
                    normalized_offer::normalize_offers(result_json);
                }
                OwnedMessage::Ping(_) => continue,
                result => {
                    println!("Unexpected type: {:?}", result);
                    break;
                }
            },
            Err(ref error) => {
                println!("ERROR: {:?}", error);
                break;
            }
        }
    }
}
