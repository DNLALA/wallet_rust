use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use wallet_core_rs::ffi::{
    TWCoinType, TWHDWallet, TWHDWalletCreateWithMnemonic, TWHDWalletDelete,
    TWHDWalletGetAddressForCoin, TWHDWalletMnemonic,
};

#[derive(Deserialize)]
struct UserPhrase {
    phrase: String,
}

#[derive(Serialize)]
struct PhraseResponse {
    received_phrase: String,
    note: String,
}

#[post("/wallets")]
async fn create_wallets(user_phrase: web::Json<UserPhrase>) -> impl Responder {
    HttpResponse::Ok().json(PhraseResponse {
        received_phrase: user_phrase.phrase.clone(),
        note: "Your phrase was received successfully".to_string(),
    })
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("🚀 Trust Wallet Core Microservice is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("📡 Server running at http://localhost:8881");

    HttpServer::new(|| App::new().service(hello).service(create_wallets))
        .bind("127.0.0.1:8881")?
        .run()
        .await
}
