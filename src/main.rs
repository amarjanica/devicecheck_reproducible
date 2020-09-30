#[macro_use]
extern crate serde_derive;

use std::{fs};
use std::time::{Duration, SystemTime};

use jsonwebtoken::{Algorithm, EncodingKey, Header};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Payload {
    device_token: String,
    transaction_id: String,
    timestamp: u128,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iat: u64,
    // Issued at (as UTC timestamp)
    pub iss: String, // Issuer
}

fn main() {
    let private_key_location = "";
    let key_id = "";
    let team_id = "";


    let private_key = fs::read_to_string(private_key_location).unwrap();
    let encoding_key = EncodingKey::from_ec_pem(private_key.as_bytes()).unwrap();

    let timestamp: Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards?");

    let mut jwt_header = Header::new(Algorithm::ES256);
    jwt_header.kid = Some(key_id.to_string());

    let claims = Claims {
        iss: team_id.to_string(),
        iat: timestamp.as_secs(),
    };

    let jwt_token = jsonwebtoken::encode(&jwt_header, &claims, &encoding_key).unwrap();

    let payload = Payload {
        device_token: "<device_token>".to_string(),
        transaction_id: Uuid::new_v4().to_string(),
        timestamp: timestamp.as_millis(),
    };

    let resp = reqwest::blocking::Client::new()
        .post("https://api.development.devicecheck.apple.com/v1/validate_device_token")
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json::<Payload>(&payload)
        .send().unwrap();

    let status = resp.status();

    let res_text = resp.text().unwrap();

    // printed result is status: 401, res: Unable to verify authorization token
    println!("status: {}, res: {:?}", status, res_text);
}
