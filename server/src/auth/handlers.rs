use axum::{extract::{State,Json},http::StatusCode};

use sqlx::PgPool;
use axum::extract::Path;
use axum::{
    
    extract::{FromRequestParts},
    http::{request::Parts},
};

use argon2::{Argon2, PasswordHasher,PasswordHash, PasswordVerifier};
use argon2::password_hash::{SaltString};
use jsonwebtoken::{encode, EncodingKey, Header};
use jsonwebtoken::{decode, DecodingKey, Validation};
use chrono::{Utc, Duration};
use std::env;
use crate::auth::models::{SignupRequest,LoginRequest,User,AuthUser,Claims, UserId};
pub async fn signup(State(pool):State<PgPool> ,Json(user): Json<SignupRequest>) -> Result<Json<User>, StatusCode> {
    let salt = b"random_salt";
    let password_hash = Argon2::default()
        .hash_password(user.password.as_bytes(), &SaltString::encode_b64(salt).unwrap())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let row: UserId = sqlx::query_as!(
    UserId,
    "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
    user.name,
    user.email,
    password_hash
)
.fetch_one(&pool)
.await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(User {
        id: row.id,
        name: user.name,
        email: user.email,
    }))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<String>, (axum::http::StatusCode, String)> {
    let user = sqlx::query!(
        "SELECT id, name, email, password_hash FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Invalid password hash".to_string()))?;


    let is_valid = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        return Err((
            axum::http::StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        ));
    }

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.email,
        exp: expiration,
        user_id: user.id,
    };
    let secret = env::var("JWT_SECRET")
    .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Missing JWT_SECRET env var".to_string()))?;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(token))
}

pub async fn get_user(
    State(pool): State<PgPool>, // DB pool from app state
    Path(user_id): Path<i32>,   // Extract ID from URL
    AuthUser(_user): AuthUser,  // <-- extractor ensures JWT is valid
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let result = sqlx::query_as!(
    User,
    "SELECT id, name, email FROM users WHERE id = $1",
    user_id
)
.fetch_one(&pool)
.await;


    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((
            axum::http::StatusCode::NOT_FOUND,
            format!("User not found: {}", e),
        )),
    }
}
pub async fn get_user_by_email(
    State(pool): State<PgPool>, // DB pool from app state
    Path(email): Path<String>,   // Extract ID from URL
    AuthUser(_user): AuthUser,  // <-- extractor ensures JWT is valid
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let result = sqlx::query_as!(
    User,
    "SELECT id, name, email FROM users WHERE email = $1",
    email
)
.fetch_one(&pool)
.await;


    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((
            axum::http::StatusCode::NOT_FOUND,
            format!("User not found for email: {}", e),
        )),
    }
}


impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts( parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;
        println!("🔍 Received Authorization header: {}", auth_header);

        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Invalid token format".to_string()));
        }

        let token = &auth_header[7..]; // remove "Bearer "
        println!("🔍 Extracted token: {}", token);

        let secret = env::var("JWT_SECRET")
    .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Missing JWT_SECRET env var".to_string()))?;

        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()), // ⚠️ should be env var
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token".to_string()))?;

        Ok(AuthUser(decoded.claims))
    }
}



