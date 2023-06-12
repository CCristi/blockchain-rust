use chrono::prelude::*;
use crate::api::run_server;
use std::sync::Mutex;
use std::thread;
use actix_web::web::service;
use once_cell::sync::Lazy;

mod block;
mod blockchain;
mod crypto_utils;
mod api;

pub(crate) static BLOCKCHAIN: Lazy<Mutex<blockchain::Blockchain>> = Lazy::new(|| Mutex::new(blockchain::Blockchain::new()));

#[actix_web::main]
async fn main() -> () {
    let server = run_server();

    // span a mine thread
    thread::spawn(move || {
        let mut i = 1;

        loop {
            let blockchain = BLOCKCHAIN.lock().unwrap();
            let last_block = (*blockchain.blocks.last().unwrap()).clone();

            drop(blockchain);

            let block = block::Block::mine_block(&last_block, format!("Block {}", i));
            let mut blockchain = BLOCKCHAIN.lock().unwrap();

            blockchain.add_block(block);
            i += 1;

            drop(blockchain);
        }
    });

    return server.await;
}
