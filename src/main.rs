use chrono::prelude::*;

mod block;
mod blockchain;
mod crypto_utils;

fn main() {
    let mut blockchain = blockchain::Blockchain::new();
    let mut prev_time: i64 = Utc::now().timestamp_millis();
    let mut duration_sum = 0;

    for i in 0..100000 {
        let block = blockchain.add_block(format!("Block {}", i));
        let mine_duration = block.timestamp - prev_time;

        duration_sum += mine_duration;
        prev_time = block.timestamp;

        println!("Block {} added, average time {}, difficulty {}", i, duration_sum / (i + 1), block.difficulty);
    }

    println!("blockchain validity: {}", blockchain.verify());
}
