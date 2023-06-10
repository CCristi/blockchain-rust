use std::fmt::Formatter;
use chrono::prelude::*;
use crate::crypto_utils;

pub struct Block {
    pub timestamp: i64,
    pub last_hash: String,
    pub hash: String,
    pub data: String,
    pub nonce: u128,
    pub difficulty: u64,
}

impl Block {
    const MINE_RATE: i64 = 1000;

    pub fn eq(&self, other: &Block) -> bool {
        self.timestamp == other.timestamp &&
            self.last_hash == other.last_hash &&
            self.hash == other.hash &&
            self.data == other.data
    }

    pub fn new(last_hash: String, data: String, timestamp: i64, nonce: u128, difficulty: u64) -> Self {
        let mut block = Block {
            timestamp,
            last_hash,
            hash: String::from(""),
            data,
            nonce,
            difficulty,
        };

        block.hash = block.compute_hash();

        block
    }

    pub fn genesis() -> Self {
        Block::new(
            String::from("genesis-last-hash"),
            String::from("Genesis Block"),
            0,
            1,
            3,
        )
    }

    pub fn mine_block(last_block: &Block, data: String) -> Self {
        let mut nonce = 0;

        loop {
            nonce += 1;

            let timestamp = Utc::now().timestamp_millis();
            let difficulty = if last_block.timestamp + Block::MINE_RATE > timestamp {
                last_block.difficulty + 1
            } else {
                last_block.difficulty - 1
            };

            let block = Block::new(
                last_block.hash.clone(),
                data.clone(),
                timestamp,
                nonce,
                difficulty,
            );

            if crypto_utils::hex_to_binary(&block.hash).starts_with(&"0".repeat(difficulty as usize)) {
                return block;
            }
        }
    }

    pub fn compute_hash(&self) -> String {
        let mut parts = String::from("");

        parts.push_str(&self.timestamp.to_string());
        parts.push_str(&self.last_hash);
        parts.push_str(&self.data);
        parts.push_str(&self.nonce.to_string());
        parts.push_str(&self.difficulty.to_string());

        crypto_utils::sha256_hash(&parts)
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block {{\n\ttimestamp: {},\n\tlast_hash: {},\n\thash: {},\n\tdata: {},\n\tnonce: {},\n\tdifficulty: {}\n}}", self.timestamp, self.last_hash, self.hash, self.data, self.nonce, self.difficulty)
    }
}
