use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: Uuid,
    pub owner_id: i32,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDocument {
    pub title: String,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDocument {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Collaborator {
    pub doc_id: Uuid,
    pub user_id: i32,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareRequest {
    pub user_id: i32,
    pub role: String, // "reader" or "editor"
}

#[derive(Debug)]
pub struct Owner {
    pub owner_id: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "role", rename_all = "lowercase")] 
pub enum Role {
    Reader,
    Editor,
}
