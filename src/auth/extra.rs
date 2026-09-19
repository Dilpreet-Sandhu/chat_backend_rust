use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};



#[derive(Serialize,Deserialize)]
pub struct Claim {

    pub sub : String,
    pub exp : u64,
    pub iat : u64
}

pub fn sign_access_token(
    user_id : uuid::Uuid,
    secret : &str
) -> Result<String,jsonwebtoken::errors::Error> {

    let cur_time = jsonwebtoken::get_current_timestamp();

    let claim = Claim {
        sub : user_id.to_string(),
        exp : cur_time + (15 * 60 * 60),
        iat : cur_time
    };

    encode(&Header::default(), &claim,&EncodingKey::from_secret(secret.as_bytes()))

}

pub fn decode_access_token(
    token : &str,
    secret : &str
) -> Result<Claim,jsonwebtoken::errors::Error> {

    let token_data = decode::<Claim>(
        token, 
        &DecodingKey::from_secret(secret.as_bytes()), 
        &Validation::new(jsonwebtoken::Algorithm::HS256)
    )?;


    Ok(
        token_data.claims
    )
}