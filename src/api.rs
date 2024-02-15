use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use jsonwebtoken::jwk::AlgorithmParameters::OctetKey;
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
    htlc_deadline_epoch: u32,
    #[serde(rename = "networkID")]
    network_id: String,
}

pub(crate) async fn message1(State(state): State<AppState>, claims: Claims, Json(request): Json<Message1>) -> (axum::http::StatusCode, Json<String>) {
    let pool = state.db;
    info!("message1 received");
    (StatusCode::OK, Json("{message: ok}".to_string()))
}