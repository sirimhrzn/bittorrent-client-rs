use core::panic;
use serde_json;
mod decoders;
use decoders::decoders::Decoder;
use std::env::{self, current_exe};

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = Decoder::new(encoded_value.to_string()).decode();
        println!("Result: {}", decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
