use std::collections::HashMap;

use axum::{async_trait, Json, RequestPartsExt};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::error::BdfError;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = dotenv::var("JWT_SECRET").unwrap();
    Keys::new(secret.as_bytes())
});

#[derive(Debug, Clone)]
struct User {
    client_id: String,
    client_secret: String,
    org: String,
}

static USER_DB: Lazy<HashMap<String, User>> = Lazy::new(|| {
    let user_db: HashMap<String, User> = HashMap::from([
        (String::from("walter"), User { client_id: String::from("walter"), client_secret: String::from("verysecurepw"), org: String::from("erste") }),
        (String::from("karl"), User { client_id: String::from("karl"), client_secret: String::from("verysecurepw"), org: String::from("rbi") }),
        (String::from("paul"), User { client_id: String::from("paul"), client_secret: String::from("verysecurepw"), org: String::from("oenb") }),
        (String::from("roman"), User { client_id: String::from("paul"), client_secret: String::from("verysecurepw"), org: String::from("erste") }),
        (String::from("johannes"), User { client_id: String::from("johannes"), client_secret: String::from("verysecurepw"), org: String::from("oenb") }),
    ]);

    user_db
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct AuthResponse {
    access_token: String,
    token_type: String,
}

impl AuthResponse {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    client_id: String,
    org: String,
    iat: i64,
    exp: i64,
}

impl Claims {
    pub fn new(client_id: String, org: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(3);
        Self {
            client_id,
            org,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
    where
        S: Send + Sync,
{
    type Rejection = BdfError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| BdfError::InvalidToken)?;
        // Decode the user data
        let token_data = jsonwebtoken::decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| BdfError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

pub(crate) async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthResponse>, BdfError> {
    debug!("authorization request");
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        debug!("Authorization Error #1");
        return Err(BdfError::InvalidCredentials);
    }

    let user_opt = USER_DB.get(&payload.client_id);

    if user_opt.is_none() {
        debug!("Authorization Error #2");
        return Err(BdfError::InvalidCredentials);
    }
    let user = user_opt.expect("Fehler hier eigentlich nicht moeglich");

    if user.client_secret != payload.client_secret {
            debug!("Authorization Error #3");
            return Err(BdfError::InvalidCredentials);
    }
    // } else {
    //     return Err(BdfError::InvalidCredentials);
    // };

    let claims = Claims::new(user.client_id.clone(), user.org.clone());
    // Create the authorization token
    let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| BdfError::TokenCreationError)?;

    // Send the authorized token
    Ok(Json(AuthResponse::new(token)))
}