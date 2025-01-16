use actix_web::{dev::ServiceRequest, Error};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

/// 認証ミドルウェア
pub struct AuthMiddleware {
    jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

impl AuthMiddleware {
    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let key = DecodingKey::from_secret(self.jwt_secret.as_bytes());
        let validation = Validation::default();
        
        let token_data = decode::<Claims>(token, &key, &validation)?;
        Ok(token_data.claims)
    }

    pub fn generate_demo_token(&self, demo_session: &DemoSession) -> Result<String, AuthError> {
        let claims = Claims {
            sub: demo_session.session_id.clone(),
            company: demo_session.company_email.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::days(14)).timestamp() as usize,
        };

        let key = EncodingKey::from_secret(self.jwt_secret.as_bytes());
        let token = encode(&Header::default(), &claims, &key)?;
        Ok(token)
    }
} 