use actix_web::HttpRequest;
use async_graphql::{Context, Guard, Result};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use sqlx::SqlitePool;
use time::{Duration, OffsetDateTime};

use crate::schema::User;

pub struct Token(pub String);

#[derive(Serialize, Deserialize)]
struct TokenUserInfo {
    id: i64,
    valid_until: OffsetDateTime,
}

const TOKEN_VALID_FOR_HOURS: i64 = 72;
pub const TOKEN_HEADER_NAME: &str = "Authorization";

fn get_signing_key() -> Hmac<Sha384> {
    Hmac::new_from_slice(b"some_secret_token").expect("Not to fail making a signing key")
}

impl Token {
    pub fn generate_token(user_id: i64) -> Result<Token, String> {
        let claims = TokenUserInfo {
            id: user_id,
            valid_until: OffsetDateTime::now_utc() + Duration::hours(TOKEN_VALID_FOR_HOURS),
        };

        let token_str = claims
            .sign_with_key(&get_signing_key())
            .map_err(|_| "Could not generate token")?;

        Ok(Token(token_str))
    }

    pub fn validate_token(&self) -> Result<i64, String> {
        let claims: TokenUserInfo = self
            .0
            .verify_with_key(&get_signing_key())
            .map_err(|_| "Authentication failed!")?;

        if claims.valid_until < OffsetDateTime::now_utc() {
            return Err("Authentication failed!".to_string());
        }

        Ok(claims.id)
    }
}

pub struct LoggedInGuard;

#[async_trait::async_trait]
impl Guard for LoggedInGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if ctx.data_opt::<User>().is_some() {
            Ok(())
        } else {
            Err("Forbidden".into())
        }
    }
}

pub async fn get_user_from_request(
    request: &HttpRequest,
    db_pool: &SqlitePool,
) -> Result<Option<User>, String> {
    let token_str = match request.headers().get(TOKEN_HEADER_NAME) {
        Some(token_header) => token_header
            .to_str()
            .map_err(|_| "Authentication failed!")?
            .to_string(),
        None => return Ok(None),
    };

    let token = Token(token_str);

    match token.validate_token() {
        Ok(user_id) => {
            let user = sqlx::query!("SELECT * FROM user WHERE id = ?", user_id)
                .fetch_one(db_pool)
                .await
                .map_err(|_| "Authentication failed!")?;

            return Ok(Some(User {
                id: user.id,
                name: user.name,
            }));
        }
        Err(_) => return Ok(None),
    }
}
