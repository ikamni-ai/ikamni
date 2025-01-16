use actix_web::{dev::ServiceRequest, Error};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

/// 認証ミドルウェア
pub struct NinshouMiddleware {
    jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenNaiyou {
    sub: String,
    kaisha: String,
    kigen: usize,
}

impl NinshouMiddleware {
    pub fn token_kakunin(&self, token: &str) -> Result<TokenNaiyou, NinshouError> {
        let key = DecodingKey::from_secret(self.jwt_secret.as_bytes());
        let validation = Validation::default();
        
        let token_data = decode::<TokenNaiyou>(token, &key, &validation)?;
        Ok(token_data.claims)
    }

    pub fn shiyou_token_sakusei(&self, session: &ShiyouSession) -> Result<String, NinshouError> {
        let naiyou = TokenNaiyou {
            sub: session.session_id.clone(),
            kaisha: session.kaisha_mail.clone(),
            kigen: (chrono::Utc::now() + chrono::Duration::days(14)).timestamp() as usize,
        };

        let key = EncodingKey::from_secret(self.jwt_secret.as_bytes());
        let token = encode(&Header::default(), &naiyou, &key)?;
        Ok(token)
    }
} 