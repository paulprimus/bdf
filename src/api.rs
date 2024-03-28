use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::authorization::Claims;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message1 {
    #[serde(rename = "TradeID")]
    trade_id: u32,
    #[serde(rename = "SellerID")]
    seller_id: String,
    #[serde(rename = "BuyerID")]
    buyer_id: String,
    #[serde(rename = "ISD")]
    isd: String,
    #[serde(rename = "Amount")]
    amount: u32,
    #[serde(rename = "HTLCSecret")]
    htlc_secret: String,
    #[serde(rename = "HTLCDeadlineEpoch")]
    htlc_deadline_epoch: i64,
    #[serde(rename = "HTLCHashFunc")]
    htlc_hash_func: String,
    #[serde(rename = "HTLCEncFunc")]
    htlc_enc_func: String,
    #[serde(rename = "networkID")]
    network_id: String,
    #[serde(rename = "SellerCBDCWalletRef")]
    seller_cbdc_wallet_ref: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message2 {
    #[serde(rename = "TradeID")]
    trade_id: u32,
    #[serde(rename = "sellerCAMBIC")]
    seller_cambic: String,
    #[serde(rename = "ISD")]
    isd: String,
    #[serde(rename = "Amount")]
    amount: u32,
    #[serde(rename = "BuyerCBDCWalletRef")]
    buyer_cbdc_wallet_ref: String,
    #[serde(rename = "HTLCHash")]
    htlc_hash: String,
    #[serde(rename = "HTLCHashFunc")]
    htlc_hash_func: String,
    #[serde(rename = "HTLCEncFunc")]
    htlc_enc_func: String,
    #[serde(rename = "networkID")]
    network_id: String,
}

pub(crate) async fn message1(State(state): State<AppState>, claims: Claims, Json(request): Json<Message1>) -> (axum::http::StatusCode, Json<String>) {
    let pool = state.db;
    info!("message1 received");
    (StatusCode::OK, Json("{message: ok}".to_string()))
}

pub(crate) async fn message2(State(state): State<AppState>, claims: Claims, Json(request): Json<Message2>) -> (axum::http::StatusCode, Json<String>) {
    let pool = state.db;
    info!("message2 received");
    (StatusCode::OK, Json("{message: ok}".to_string()))
}
