use axum::http::StatusCode;
use chrono::{DateTime, Duration, Utc};
use dotenv::dotenv;
use std::env;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData};
use crate::core::contracts::user::TokenClaims;

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let now: DateTime<Utc> = Utc::now();
    let expire: Duration = Duration::hours(24);
    
    let claim: TokenClaims = TokenClaims{
        email,
        iat: now.timestamp() as usize,
        exp: (now+expire).timestamp() as usize,
    };

    dotenv().ok();
    let secret: String = env::var("JWT_SECRET").unwrap().clone();

    let encoded =  encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_| {StatusCode::INTERNAL_SERVER_ERROR});
    return encoded
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<TokenClaims>, StatusCode> {
    dotenv().ok();
    let secret: String = env::var("JWT_SECRET").unwrap().clone();
    let res: Result<TokenData<TokenClaims>, StatusCode> = decode(&jwt, &DecodingKey::from_secret(secret.as_ref()), &Default::default())
        .map_err(|_| {StatusCode::INTERNAL_SERVER_ERROR});
    return res
}