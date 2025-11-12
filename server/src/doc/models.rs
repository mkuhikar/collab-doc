use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use std::str::FromStr;
use tokio::{sync::{broadcast, Mutex}, time::Instant};
use std::{net::SocketAddr, sync::Arc, time::Duration};

use dashmap::DashMap;

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
    pub email: String, // <-- changed from user_id to email
    pub role: Role,    // "reader" or "editor"
}


#[derive(Debug,FromRow)]
pub struct Owner {
    pub owner_id: i32,
}

// Simple Operation types for plain text
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Op {
    Insert { pos: usize, text: String },
    Delete { pos: usize, len: usize },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientMessage {
    pub client_id: String,      // <-- add this
    pub client_version: u64,
    pub op: Op,
}

// server -> clients broadcasts
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ServerMessage {
    pub client_id: String,      // <-- add this (sender id)
    pub version: u64,
    pub op: Op,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "role", rename_all = "lowercase")] 
pub enum Role {
    Reader,
    Editor,
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "reader" => Ok(Role::Reader),
            "editor" => Ok(Role::Editor),
            _ => Err(format!("Invalid role: {}", s)),
        }
    }
}





