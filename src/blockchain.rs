use std::time::Instant;
use sha256::{digest};

#[derive(Debug)] //<- println!
pub struct Block {
    index: u32,
    timestamp: u128,
    data: String,
    prev_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u32, data: String, prev_hash: String) -> Block {
        let timestamp = Instant::now().elapsed().as_millis();
        let hash = calculate_hash(index, &data, &prev_hash, timestamp);
        
        Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
        }
    }
}

fn calculate_hash(index: u32, data: &String, prev_hash: &String, timestamp: u128) -> String {
    let input = index.to_string() + &data + &prev_hash + &timestamp.to_string();
    digest(input)
}