use chrono::prelude::*;
use jsonwebtoken::{
    decode, encode, errors::Result as JWTResult, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct JWT {
    /// user id
    sub: i32,
    /// issued at (in UTC timestamp)
    iat: i64,
    /// expire at (in UTC timestamp). Validated
    pub(self) exp: i64,
}

impl JWT {
    pub fn new(user_id: i32, duration: i64) -> Self {
        let now = Utc::now();
        let ts = now.timestamp();
        JWT {
            sub: user_id,
            iat: ts,
            exp: ts + duration,
        }
    }

    pub fn encode(&self, secret: &str) -> JWTResult<String> {
        encode(
            &Header::new(Algorithm::HS256),
            &self,
            &EncodingKey::from_base64_secret(secret)?,
        )
    }

    pub fn from_token(token: &str, secret: &str) -> JWTResult<Self> {
        decode::<JWT>(
            token,
            &DecodingKey::from_base64_secret(secret)?,
            &Validation::default(),
        )
        .map(|data| data.claims)
    }

    pub fn user_id(&self) -> i32 {
        self.sub
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_jwt() {
        let now = Utc::now().timestamp();
        let jwt = JWT::new(1, 600);
        assert!((jwt.iat - now).abs() <= 1);

        // encode
        let secret = "Jz86qAemE7w9SiWj2Nda66+xTN0GFCNJZoPVp5fg/P8=";
        let encoded = jwt.encode(&secret);
        assert!(encoded.is_ok());
        let token = encoded.unwrap();
        // decode
        let jwt = JWT::from_token(&token, &secret).unwrap();
        assert_eq!(jwt.sub, 1);

        // modify one digit can it should fail to decode
        assert!(JWT::from_token(&token[..token.len() - 1], &secret).is_err());
        assert!(JWT::from_token(&token[1..], &secret).is_err());

        // validate exp
        let mut jwt = JWT::new(1, 10);
        jwt.exp = now - 10;
        let token = jwt.encode(&secret).unwrap();
        assert!(JWT::from_token(&token[1..], &secret).is_err());
    }
}
