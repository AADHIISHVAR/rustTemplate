use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{HttpResponse, Responder};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::models::jwt_claims::JwtClaims;

pub fn user_jwt_encoder(enterfield: String, role: String) -> impl Responder {
    // 1. Get current UNIX timestamp and add 1 hour (3600 seconds)
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600;

    // 2. Build the claims struct
    let claims = JwtClaims {
        enterfield: enterfield.to_string(),
        roles: role.to_string(),
        exp: exp as usize, // JWT expects this as usize
    };

    // 3. Define the secret key used for encoding/decoding
    let secret = "Hello ,World!";

    // 4. Encode the claims into a JWT token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).expect("Error encoding JWT(token)");

    println!("Encoded token: {}", token);

    // 5. Decode the token just to demonstrate (education only)
    let token_data = decode::<JwtClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    // 6. Handle the result using match
    match token_data {
        Ok(data) => {
            println!("✅ Token is valid. Claims: {:?}", data.claims);
        }
        Err(err) => {
            println!("❌ Invalid token: {}", err);
        }
    }

    // 7. Respond to the client with the token
    HttpResponse::Ok().body(token)
}
