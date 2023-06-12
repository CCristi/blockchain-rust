use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::{BLOCKCHAIN, blockchain};


#[get("/api/blocks")]
async fn blocks() -> impl Responder {
    let blockchain = BLOCKCHAIN.lock().unwrap();

    HttpResponse::Ok().body(format!("{:?}", blockchain))
}

pub async fn run_server() {
    HttpServer::new(|| {
        App::new()
            .service(blocks)
    })
        .bind(("127.0.0.1", 3000)).expect("Can not bind to port 3000")
        .run()
        .await
        .unwrap();
}


