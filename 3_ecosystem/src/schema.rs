use async_graphql::{Context, EmptySubscription, Object, Result, Schema};
use sqlx::SqlitePool;

use crate::{
    auth::{LoggedInGuard, Token},
    hashing::{hash_password, validate_hash},
};

pub type ServerSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct User {
    pub id: i64,
    pub name: String,
}

#[Object]
impl User {
    pub async fn id(&self) -> i64 {
        self.id
    }

    pub async fn name(&self) -> &str {
        &self.name
    }

    pub async fn friends<'a>(&self, ctx: &Context<'a>) -> Result<Vec<User>> {
        let db_pool = ctx.data::<SqlitePool>()?;
        let user_id = self.id;

        let res = sqlx::query!(
            r#"
            SELECT user.* FROM user_friends
            LEFT JOIN user on user.id = user_friends.friend_id
            WHERE user_friends.user_id = ?
            "#,
            user_id
        )
        .fetch_all(db_pool)
        .await?;

        Ok(res
            .into_iter()
            .map(|entry| User {
                id: entry.id,
                name: entry.name,
            })
            .collect())
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    #[graphql(guard = "LoggedInGuard")]
    pub async fn user<'a>(&self, ctx: &Context<'a>, user_id: Option<i64>) -> Result<User> {
        let db_pool = ctx.data::<SqlitePool>()?;
        // fine to `ctx.data::<User>()?` because the query is login guarded
        let user_id = user_id.unwrap_or(ctx.data::<User>()?.id);

        let res = sqlx::query!("SELECT * FROM user WHERE id = ?", user_id)
            .fetch_one(db_pool)
            .await?;

        Ok(User {
            id: res.id,
            name: res.name,
        })
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn register<'a>(
        &self,
        ctx: &Context<'a>,
        user_name: String,
        password: String,
    ) -> Result<bool> {
        let db_pool = ctx.data::<SqlitePool>()?;

        let pw_hash = hash_password(password)?;

        sqlx::query!(
            "INSERT INTO user (name, pwd_hash) VALUES (?, ?) RETURNING id",
            user_name,
            pw_hash
        )
        .fetch_one(db_pool)
        .await?;

        Ok(true)
    }

    pub async fn login<'a>(
        &self,
        ctx: &Context<'a>,
        user_name: String,
        password: String,
    ) -> Result<String> {
        let db_pool = ctx.data::<SqlitePool>()?;

        let user = sqlx::query!("SELECT * FROM user WHERE name = ?", user_name)
            .fetch_one(db_pool)
            .await?;

        if validate_hash(user.pwd_hash, password)? {
            let token = Token::generate_token(user.id)?;

            return Ok(token.0);
        } else {
            return Err("Authentication failed!".into());
        }
    }

    #[graphql(guard = "LoggedInGuard")]
    pub async fn add_friend<'a>(&self, ctx: &Context<'a>, new_friend_id: i64) -> Result<bool> {
        let db_pool = ctx.data::<SqlitePool>()?;
        let user_id = ctx.data::<User>()?.id;

        sqlx::query!(
            "INSERT INTO user_friends VALUES (?, ?)",
            user_id,
            new_friend_id
        )
        .execute(db_pool)
        .await?;

        Ok(true)
    }

    #[graphql(guard = "LoggedInGuard")]
    pub async fn remove_friend<'a>(&self, ctx: &Context<'a>, new_friend_id: i64) -> Result<bool> {
        let db_pool = ctx.data::<SqlitePool>()?;
        let user_id = ctx.data::<User>()?.id;

        sqlx::query!(
            "DELETE FROM user_friends WHERE user_id = ? AND friend_id = ?",
            user_id,
            new_friend_id
        )
        .execute(db_pool)
        .await?;

        Ok(true)
    }
}
