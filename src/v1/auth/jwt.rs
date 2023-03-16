use jsonwebtoken::{encode, Header, EncodingKey, errors::Error, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use chrono::offset::Utc;

const ACCESS_TOKEN_EXPIRATION: usize = 3600;
const REFRESH_TOKEN_EXPIRATION: usize = 604800;
const ACCESS_SIGNING_KEY: &str = "access_secret"; //replace this with better secret eventually
const REFRESH_SIGNING_KEY: &str = "refresh_secret"; //replace this with better secret eventually


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize,          // Optional. Issued at (as UTC timestamp)
    pub sub: String,         // Optional. Subject (whom token refers to)
}

impl Claims {
    fn new(exp_offset: usize, sub: &str) -> Self {
        Claims {
            exp: Utc::now().timestamp() as usize + exp_offset,
            iat: Utc::now().timestamp() as usize,
            sub: sub.to_string(),
        }
    }    

    fn to_token(&self, key: &str) -> Result<String, Error> {
        let token = encode(
            &Header::default(), 
            &self, 
            &EncodingKey::from_secret(key.as_ref())
        )?;
        Ok(token)
    }
}

pub fn create_access_token(subject: &str) -> Result<String, Error> {
    let claims = Claims::new(ACCESS_TOKEN_EXPIRATION, subject);
    let token = claims.to_token(ACCESS_SIGNING_KEY)?;
    Ok(token)
}

pub fn create_refresh_token(subject: &str) -> Result<String, Error> {
    let claims = Claims::new(REFRESH_TOKEN_EXPIRATION, subject);
    let token = claims.to_token(REFRESH_SIGNING_KEY)?;
    Ok(token)
}

pub fn verify_access_token(token: &str) -> Result<Claims, Error> {
    let token = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(ACCESS_SIGNING_KEY.as_ref()),
        &Validation::default()
    )?;
    Ok(token.claims)
}

pub fn verify_refresh_token(token: &str) -> Result<Claims, Error> {
    let token = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(REFRESH_SIGNING_KEY.as_ref()),
        &Validation::default()
    )?;
    Ok(token.claims)
}