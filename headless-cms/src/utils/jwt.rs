use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

pub fn test() {
    let my_claims: Claims;

    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some("blabla".to_owned());
    let token = encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;
}
