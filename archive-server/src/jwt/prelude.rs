use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Jwt {
    exp: usize,
    iat: usize,
    pub username: String,
}

const ENCODING_KEY: &str = "ABCABCABCABCABCABC";

impl Jwt {
    pub fn new(username: String) -> Self {
        let iat = jsonwebtoken::get_current_timestamp() as usize;
        let exp = iat + 10;
        Self { username, exp, iat }
    }

    fn decoding_key() -> DecodingKey {
        DecodingKey::from_secret(ENCODING_KEY.as_bytes())
    }

    fn encoding_key() -> EncodingKey {
        EncodingKey::from_secret(ENCODING_KEY.as_bytes())
    }

    fn header() -> Header {
        Header::default()
    }

    fn validation() -> Validation {
        let mut validation = Validation::default();
        validation.leeway = 0;
        validation
    }

    pub fn decode(token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        jsonwebtoken::decode::<Self>(token, &Self::decoding_key(), &Self::validation())
            .map(|result| result.claims)
            .map_err(|error| error.into())
    }

    pub fn encode(&self) -> Result<String, Box<dyn std::error::Error>> {
        jsonwebtoken::encode(&Self::header(), self, &Self::encoding_key())
            .map_err(|error| error.into())
    }
}
