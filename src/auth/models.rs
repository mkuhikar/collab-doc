use serde::{Deserialize,Serialize};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}
#[derive(Debug,Deserialize,Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}
#[derive(sqlx::FromRow)]
pub struct UserId {
    pub id: i32,
}



#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize,serde::Deserialize)]
pub struct Claims {
    pub sub: String,   // subject (user email)
    pub user_id: i32,
    pub exp: usize,    // expiration
    
}


pub struct AuthUser(pub Claims);


